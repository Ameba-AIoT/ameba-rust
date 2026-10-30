use std::collections::HashSet;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};

use ameba_flash::elf::package_image2;
use ameba_flash::flash::session::flash_session;
use ameba_flash::monitor::{LOGUART_BAUD, monitor_serial};
use ameba_flash::{pack, prebuilt};
use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use probe_rs::MemoryInterface;
use probe_rs::config::Registry;
use probe_rs::semihosting::SemihostingCommand;
use probe_rs::{BreakpointCause, CoreStatus, HaltReason, Permissions, Session};

#[derive(Parser)]
#[command(name = "xtask", about = "Ameba Rust SDK build tasks")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// Restrict `check` / `lint` / `fmt` to a subset of crates.
///
/// Omit to operate on every crate (the current behavior).
#[derive(clap::Args, Clone)]
struct PackagesArg {
    /// Comma-separated crate labels (e.g. ameba-hal,hello-world). Labels are
    /// the same names printed by `check`/`lint`/`fmt`'s per-crate progress
    /// lines.
    #[arg(long, value_delimiter = ',')]
    packages: Vec<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Build firmware (example / all examples / hil-test / documentation)
    Build {
        #[command(subcommand)]
        target: BuildTarget,
    },
    /// Run an example: build → package → UART flash → serial monitor
    Run {
        #[command(subcommand)]
        target: RunTarget,
    },
    /// cargo check all firmware crates
    Check {
        #[command(flatten)]
        packages: PackagesArg,
    },
    /// cargo clippy all firmware crates
    Lint {
        /// Automatically apply fixes (clippy --fix)
        #[arg(long)]
        fix: bool,
        #[command(flatten)]
        packages: PackagesArg,
    },
    /// cargo fmt all firmware crates
    Fmt {
        /// Check only, do not write changes (for CI)
        #[arg(long)]
        check: bool,
        #[command(flatten)]
        packages: PackagesArg,
    },
    /// Clean build artifacts
    Clean {
        /// Also delete the Cargo target/ directory
        #[arg(long)]
        all: bool,
    },
    /// CI: check + lint + fmt --check + build doc + build hil-test binaries
    ///
    /// Collects failures and reports a summary (does not stop at the first).
    Ci {
        /// Run only these steps (check, lint, fmt, docs, tests)
        #[arg(long, value_delimiter = ',')]
        steps: Vec<String>,
        /// Skip the clippy step
        #[arg(long)]
        no_lint: bool,
        /// Skip the documentation step
        #[arg(long)]
        no_docs: bool,
    },
    /// [P1] Package and flash a hil-test ELF, then monitor semihosting.
    ///
    /// Intended as cargo runner:
    ///   .cargo/config.toml:  runner = "cargo xtask run-hil-test"
    ///   cargo test --bin gpio
    ///
    /// The ELF path is passed by cargo as the trailing positional argument.
    RunHilTest {
        /// ELF path (from cargo runner)
        elf: PathBuf,
        /// Path to the RTL8721F chip YAML for probe-rs
        /// (relative paths resolve against the repo root)
        #[arg(long, default_value = "probe-rs-chips/RTL8721F.yaml")]
        chip: PathBuf,
        /// Flash base address for app.bin
        #[arg(long, default_value_t = FLASH_APP_BASE)]
        flash_base: u32,
    },
    /// Flash an existing app.bin and monitor semihosting.
    FlashRun {
        /// Path to pre-built app.bin
        #[arg(long)]
        app: PathBuf,
        /// Path to boot.bin (image1); defaults to the staged
        /// `ameba-rom-sys/bin/rtl8721f/boot.bin` from the last build
        #[arg(long)]
        boot: Option<PathBuf>,
        /// Path to RTL8721F chip YAML
        /// (relative paths resolve against the repo root)
        #[arg(long)]
        chip: PathBuf,
        /// Flash base address
        #[arg(long, default_value_t = FLASH_APP_BASE)]
        flash_base: u32,
    },
    /// Regenerate the RTL8721F PAC from svd/ (svdtools → svd2rust → form).
    ///
    /// Inputs are the two committed files under
    /// `ameba-hal/pac/rtl8721f/svd/`: `rtl8721f.base.svd` (what the hspec
    /// pipeline should emit) and `rtl8721f.yaml` (everything that pipeline
    /// structurally cannot emit). Outputs are `lib.rs`, `device.x` and `src/`.
    GenPac {
        /// Compare against the committed PAC and fail on any difference,
        /// without writing anything (CI / pre-commit gate).
        #[arg(long)]
        check: bool,
    },
}

#[derive(Subcommand)]
enum BuildTarget {
    /// Build a single example → app.bin
    Example {
        /// Example name (directory under examples/)
        #[arg(long, default_value = "hello-world")]
        name: String,
    },
    /// Build all examples → app.bin (one per example)
    Examples,
    /// Build hil-test binary/binaries
    Tests {
        /// Binary name inside hil-test/src/bin/ (without .rs); omit to build all
        #[arg(long)]
        name: Option<String>,
    },
    /// Generate API documentation for all firmware crates
    ///
    /// Pure rustdoc step: builds no firmware image.
    Doc,
}

#[derive(Subcommand)]
enum RunTarget {
    /// Build an example, flash via UART, then monitor serial
    Example {
        /// Example name (directory under examples/)
        #[arg(long, default_value = "hello-world")]
        name: String,
        /// Serial port (or RTK_SERIAL_PORT env var)
        #[arg(long)]
        port: Option<String>,
        /// Serial monitor baud rate
        #[arg(long, default_value_t = LOGUART_BAUD)]
        baud: u32,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Build { target } => {
            let root = repo_root();
            match target {
                BuildTarget::Example { name } => {
                    let crate_dir = root.join("examples").join(&name);
                    if !crate_dir.join("Cargo.toml").is_file() {
                        bail!(
                            "example {name:?} not found: {} has no Cargo.toml\n\
                             Run `cargo xtask build examples` to see the discovered list.",
                            crate_dir.display()
                        );
                    }
                    build_firmware(&name.replace('/', "-"), &crate_dir)
                }
                BuildTarget::Examples => {
                    let examples = discover_examples()?;
                    build_batch("example", &examples)
                }
                BuildTarget::Tests { name } => {
                    let crate_dir = root.join("hil-test");
                    match name {
                        Some(name) => build_firmware(&name, &crate_dir),
                        None => {
                            let bins: Vec<(String, PathBuf)> = discover_hil_bins()?
                                .into_iter()
                                .map(|b| (b, crate_dir.clone()))
                                .collect();
                            build_batch("hil-test bin", &bins)
                        }
                    }
                }
                BuildTarget::Doc => build_doc(),
            }
        }
        Commands::Run { target } => match target {
            RunTarget::Example { name, port, baud } => run_example(&name, port.as_deref(), baud),
        },
        Commands::Check { packages } => run_check(&packages.packages),
        Commands::Lint { fix, packages } => run_lint(fix, &packages.packages),
        Commands::Fmt { check, packages } => run_fmt(check, &packages.packages),
        Commands::Clean { all } => run_clean(all),
        Commands::Ci {
            steps,
            no_lint,
            no_docs,
        } => run_ci(steps, no_lint, no_docs),
        Commands::RunHilTest {
            elf,
            chip,
            flash_base,
        } => {
            let root = repo_root();
            let build_dir = root.join("build").join("rtl8721f");
            let prebuilt = root.join("ameba-rom-sys").join("bin").join("rtl8721f");

            std::fs::create_dir_all(&build_dir)?;

            let km4ns_bin = stage_prebuilts(&prebuilt, &build_dir)?;

            let km4tz_bin = build_dir.join("km4tz_image2_all.bin");
            package_image2(&elf, &km4tz_bin)?;

            let app_bin = build_dir.join("app.bin");
            pack::fw_pack(&app_bin, &km4ns_bin, &km4tz_bin, &build_dir)?;

            let chip_yaml = normalize_yaml_path(&chip, &root);
            let boot_bin = prebuilt.join("boot.bin");
            run_hil_test_session(&boot_bin, &app_bin, &chip_yaml, flash_base)
        }
        Commands::FlashRun {
            app,
            boot,
            chip,
            flash_base,
        } => {
            let chip_yaml = normalize_yaml_path(&chip, &repo_root());
            let boot_bin = boot.unwrap_or_else(|| {
                repo_root()
                    .join("ameba-rom-sys")
                    .join("bin")
                    .join("rtl8721f")
                    .join("boot.bin")
            });
            run_hil_test_session(&boot_bin, &app, &chip_yaml, flash_base)
        }
        Commands::GenPac { check } => run_gen_pac(check),
    }
}

// ── constants ────────────────────────────────────────────────────────────────

const RUST_TARGET: &str = "thumbv8m.main-none-eabi";

/// Valid `ci --steps` ids, in execution order.
const CI_STEPS: &[&str] = &["check", "lint", "fmt", "docs", "tests", "pac"];

// ── probe-rs SWD flash addresses (CPU-XIP addressing) ─────────────────────────
/// Flash load address of the packaged `app.bin` (same base as `FlashRun`/`RunHilTest`).
const FLASH_APP_BASE: u32 = 0x0404_0000;
/// Flash load address of `boot.bin` (image1), 256 KB below `FLASH_APP_BASE` so the two
/// images sit back to back. Confirmed by decrypting the real `RTL8721F_NOR.rdev` device
/// profile (`boot.bin` 0x08000000-0x08040000, `app.bin` 0x08040000-0x081c0000 in the
/// profile's own raw-flash addressing) and cross-checking the fixed 0x04000000 offset
/// between that scheme and `FLASH_APP_BASE`.
const FLASH_BOOT_BASE: u32 = 0x0400_0000;

// ── crate lists for check / lint / fmt ───────────────────────────────────────

/// Top-level directories that hold a `Cargo.toml` but are not firmware crates.
///
/// `xtask` and `ameba-flash` are host crates (added separately, without
/// `--target`); `examples`
/// is a directory of crates discovered by [`discover_examples`]. The generated
/// PAC is absent from this top-level scan because it lives nested under
/// `ameba-hal/pac/rtl8721f`, not at the repo root — it is instead pushed
/// explicitly in [`all_crate_targets`] (with `skip_fmt: true`, since `gen-pac`
/// owns its formatting via step 4's own rustfmt pass over every register
/// module; letting `fmt` reach it too would risk `gen-pac --check` reporting a
/// stale PAC over a pure formatting difference).
const NON_FIRMWARE_DIRS: &[&str] = &["xtask", "examples", "ameba-flash"];

/// Library/test firmware crates (not examples, not xtask), cross-compiled for
/// `RUST_TARGET`.
///
/// Discovered by scanning top-level directories for a `Cargo.toml` rather than
/// from a hardcoded list: a new crate would otherwise have to be registered by
/// hand, and forgetting silently drops it from `check` / `lint` / `fmt` /
/// `build doc` with no error. Scanning
/// also covers the reverse case — `hil-test` is absent from some distributions
/// of this repo, so callers must not assume any given crate is present.
///
/// Sorted by name for deterministic ordering across platforms.
fn existing_firmware_crate_targets(root: &Path) -> Vec<CrateTarget> {
    let Ok(entries) = std::fs::read_dir(root) else {
        return Vec::new();
    };

    let mut targets: Vec<CrateTarget> = entries
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.is_dir() && p.join("Cargo.toml").is_file())
        .filter_map(|p| {
            let label = p.file_name()?.to_str()?.to_string();
            (!NON_FIRMWARE_DIRS.contains(&label.as_str())).then_some(CrateTarget {
                label,
                dir: p,
                host: false,
                extra_args: &[],
                skip_fmt: false,
            })
        })
        .collect();

    targets.sort_by(|a, b| a.label.cmp(&b.label));
    targets
}

/// One crate to check / lint / format.
#[derive(Clone)]
struct CrateTarget {
    /// Display label (also the cargo bin name for examples).
    label: String,
    dir: PathBuf,
    /// Host crate (xtask): built for the host triple, no `--target`.
    host: bool,
    /// Extra args always passed to `cargo` for this target, e.g. the
    /// generated PAC's non-default `rt` feature (`[features] rt = []`),
    /// which must be forced on for `check`/`lint` to see its real-world
    /// `#[cfg(feature = "rt")]` surface (IRQ externs, `steal()` wiring).
    extra_args: &'static [&'static str],
    /// Excluded from `fmt`: the generated PAC is formatted by `gen-pac`'s
    /// own `rustfmt_register_modules()` pass (see [`NON_FIRMWARE_DIRS`]),
    /// so running the generic `cargo fmt` over it too would make
    /// `gen-pac --check` misreport pure formatting drift as a stale PAC.
    skip_fmt: bool,
}

/// Every crate `check` / `lint` / `fmt` operate on: firmware libs, all
/// examples, xtask, then the generated PAC.
///
/// Order matters: libraries first so a library break is reported before the
/// 32 examples that would each re-report it.
fn all_crate_targets() -> Result<Vec<CrateTarget>> {
    let root = repo_root();
    let mut targets: Vec<CrateTarget> = existing_firmware_crate_targets(&root);

    for (label, dir) in discover_examples()? {
        targets.push(CrateTarget {
            label,
            dir,
            host: false,
            extra_args: &[],
            skip_fmt: false,
        });
    }

    for host_crate in ["ameba-flash", "xtask"] {
        targets.push(CrateTarget {
            label: host_crate.to_string(),
            dir: root.join(host_crate),
            host: true,
            extra_args: &[],
            skip_fmt: false,
        });
    }

    // Not reachable by the top-level scan in `existing_firmware_crate_targets`
    // (it lives nested under `ameba-hal/pac/`), so it is pushed explicitly
    // here instead, mirroring the hardcoded `xtask` entry above.
    targets.push(CrateTarget {
        label: "rtl8721f-pac".to_string(),
        dir: root.join("ameba-hal/pac/rtl8721f"),
        host: false,
        extra_args: &["--features", "rt"],
        skip_fmt: true,
    });

    Ok(targets)
}

/// Run a cargo subcommand in one crate directory.
///
/// `pre` are args before `--`, `post` are args after `--` (lint flags).
/// Cross-compiled crates get `--target`; host crates do not. The target's
/// own `extra_args` (e.g. a forced feature flag) are appended after `pre`.
///
/// Errors carry the crate path as context, so a spawn failure is never
/// silently reported as a lint failure.
fn cargo_in(target: &CrateTarget, subcommand: &str, pre: &[&str], post: &[&str]) -> Result<()> {
    let mut cmd = Command::new("cargo");
    cmd.arg(subcommand);

    if !target.host {
        cmd.args(["--target", RUST_TARGET]);
    }
    cmd.args(pre);
    cmd.args(target.extra_args);
    if !post.is_empty() {
        cmd.arg("--").args(post);
    }
    cmd.current_dir(&target.dir);

    run(&mut cmd).with_context(|| {
        format!(
            "`cargo {subcommand}` failed in {} ({})",
            target.label,
            target.dir.display()
        )
    })
}

// ── helpers ──────────────────────────────────────────────────────────────────

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask crate has workspace parent")
        .to_path_buf()
}

fn run(cmd: &mut Command) -> Result<()> {
    let display = format!("{:?}", cmd);
    println!("[run] {}", display);
    let status = cmd
        .status()
        .with_context(|| format!("failed to spawn: {display}"))?;
    if !status.success() {
        bail!(
            "command failed (exit {}): {}",
            status.code().unwrap_or(-1),
            display
        );
    }
    Ok(())
}

// ── example / hil-test discovery ─────────────────────────────────────────────

/// Return (bin_name, crate_dir) for every buildable example.
fn discover_examples() -> Result<Vec<(String, PathBuf)>> {
    let root = repo_root();
    let examples_root = root.join("examples");
    let mut result = Vec::new();

    for entry in std::fs::read_dir(&examples_root)
        .with_context(|| format!("failed to read {}", examples_root.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();

        if path.join("Cargo.toml").is_file() {
            // Flat example (e.g. examples/hello-world/)
            result.push((name, path));
        } else {
            // Nested example group (e.g. examples/gpio/basic/)
            let rd = std::fs::read_dir(&path);
            if let Ok(rd) = rd {
                for sub in rd.filter_map(|e| e.ok()) {
                    let sub_path = sub.path();
                    if sub_path.is_dir() && sub_path.join("Cargo.toml").is_file() {
                        let sub_name = sub_path
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("")
                            .to_string();
                        let bin_name = format!("{}-{}", name, sub_name);
                        result.push((bin_name, sub_path));
                    }
                }
            }
        }
    }

    result.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(result)
}

/// Return binary names for all hil-test/*.rs files.
///
/// `hil-test/` is absent from some distributions of this repo (e.g. the
/// public release, which excludes the still-unverified HIL framework), so a
/// missing directory is "no binaries" rather than an error.
fn discover_hil_bins() -> Result<Vec<String>> {
    let root = repo_root();
    let bin_dir = root.join("hil-test").join("src").join("bin");
    if !bin_dir.is_dir() {
        return Ok(Vec::new());
    }
    let mut names = Vec::new();

    for entry in std::fs::read_dir(&bin_dir)
        .with_context(|| format!("failed to read {}", bin_dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("rs")
            && let Some(stem) = path.file_stem().and_then(|s| s.to_str())
        {
            names.push(stem.to_string());
        }
    }

    names.sort();
    Ok(names)
}

// ── build_firmware ─────────────────────────────────────────────────────────

fn build_firmware(bin_name: &str, crate_dir: &Path) -> Result<()> {
    let root = repo_root();
    let build_dir = root.join("build").join("rtl8721f");
    let prebuilt = root.join("ameba-rom-sys").join("bin").join("rtl8721f");

    std::fs::create_dir_all(&build_dir)?;

    let km4ns_bin = stage_prebuilts(&prebuilt, &build_dir)?;
    check_np_api_ids(&prebuilt, &root.join("ameba-wifi").join("src"))?;
    let elf = build_rust_binary(bin_name, crate_dir, &build_dir)?;

    let km4tz_bin = build_dir.join("km4tz_image2_all.bin");
    package_image2(&elf, &km4tz_bin)?;

    let app_bin = build_dir.join("app.bin");
    pack::fw_pack(&app_bin, &km4ns_bin, &km4tz_bin, &build_dir)?;

    let ota_bin = build_dir.join("ota_all.bin");
    pack::ota_prepend_header(&app_bin, &ota_bin)?;

    let size = std::fs::metadata(&app_bin)?.len();
    let ota_size = std::fs::metadata(&ota_bin)?.len();
    println!("\n[done] app.bin → {}  ({} bytes)", app_bin.display(), size);
    println!(
        "[done] ota_all.bin → {}  ({} bytes)",
        ota_bin.display(),
        ota_size
    );
    Ok(())
}

/// Build every `(bin_name, crate_dir)` in turn, collecting failures so one
/// broken crate does not hide the state of the rest, then report `N/M`.
///
/// `build_firmware` always writes the same `build/rtl8721f/app.bin`, so each
/// image is copied aside to `build/rtl8721f/<kind>/<bin_name>.bin` — otherwise
/// a batch build would silently leave only the last item's artifact behind.
fn build_batch(kind: &str, items: &[(String, PathBuf)]) -> Result<()> {
    let out_dir = repo_root().join("build").join("rtl8721f").join("artifacts");
    std::fs::create_dir_all(&out_dir)?;

    println!("[xtask] building {} {kind}(s)", items.len());
    let mut failed = Vec::new();

    for (i, (bin_name, crate_dir)) in items.iter().enumerate() {
        println!(
            "\n[xtask] === ({}/{}) {kind}: {bin_name} ===",
            i + 1,
            items.len()
        );
        let result = build_firmware(bin_name, crate_dir);
        match result {
            Ok(()) => {
                let build_dir = repo_root().join("build").join("rtl8721f");
                for (file, suffix) in [("app.bin", ".bin"), ("ota_all.bin", ".ota.bin")] {
                    let src = build_dir.join(file);
                    let dst = out_dir.join(format!("{bin_name}{suffix}"));
                    if let Err(e) = std::fs::copy(&src, &dst) {
                        eprintln!("[xtask] warning: could not keep {}: {e}", dst.display());
                    } else {
                        println!("[xtask] kept {}", dst.display());
                    }
                }
            }
            Err(e) => {
                eprintln!("[xtask] FAIL {bin_name}: {e:?}");
                failed.push(bin_name.clone());
            }
        }
    }

    let ok = items.len() - failed.len();
    if failed.is_empty() {
        println!(
            "\n[done] {ok}/{} {kind}(s) compiled successfully",
            items.len()
        );
        Ok(())
    } else {
        println!(
            "\n[xtask] {ok}/{} {kind}(s) succeeded; failures:",
            items.len()
        );
        for f in &failed {
            println!("  - {f}");
        }
        bail!(
            "{} of {} {kind}(s) failed: {}",
            failed.len(),
            items.len(),
            failed.join(", ")
        )
    }
}

/// Copy the committed NP image into `build/` and hand back its path.
///
/// `boot.bin` and `km4ns_image2_all.bin` are committed under
/// `ameba-rom-sys/bin/rtl8721f/`, pinned to one C SDK revision. Building,
/// packaging and flashing read them from there.
/// Nothing here reaches outside the repo: a missing file is a repo problem, not
/// something to paper over by copying from an SDK, which would let the committed
/// copy drift from the pinned revision.
///
/// Only the NP image is copied into `build/`; `boot.bin` is read from the
/// committed path directly by the flashing commands.
fn stage_prebuilts(prebuilt: &Path, build_dir: &Path) -> Result<PathBuf> {
    for name in ["km4ns_image2_all.bin", "boot.bin"] {
        let p = prebuilt.join(name);
        if !p.is_file() {
            bail!(
                "committed prebuilt missing: {}\n\
                 This file ships committed in the repository — see \
                 ameba-rom-sys/README.md for what it is. The checkout may be \
                 incomplete.",
                p.display()
            );
        }
    }

    let km4ns_bin = build_dir.join("km4ns_image2_all.bin");
    if !files_equal(&prebuilt.join("km4ns_image2_all.bin"), &km4ns_bin)? {
        std::fs::copy(prebuilt.join("km4ns_image2_all.bin"), &km4ns_bin)?;
    }
    Ok(km4ns_bin)
}

/// Fail the build if `ameba-wifi` calls an `api_id` the pinned NP image cannot
/// dispatch.
///
/// The NP image is prebuilt, so nothing at compile time ties an `api_id`
/// reachable from Rust to one NP implements. A call to an unsupported id
/// compiles and links cleanly and fails only on hardware, where NP never clears
/// `api_id` and `send_api_req` spins until the caller's timeout — with no output
/// naming the cause. This check reports that failure at build time.
///
/// `np_api_ids.txt` is committed next to the image and regenerated with it.
fn check_np_api_ids(prebuilt: &Path, wifi_src: &Path) -> Result<()> {
    let list = prebuilt.join("np_api_ids.txt");
    if !list.is_file() || !wifi_src.is_dir() {
        // Nothing to check against (e.g. a chip whose list isn't generated yet).
        // Staying quiet here is safe: `stage_prebuilts` already hard-fails on a
        // missing image, so this can only be a list-only gap.
        return Ok(());
    }

    let supported = parse_np_api_ids(&list)?;
    let rust_ids = parse_rust_api_ids(&wifi_src.join("whc").join("msg.rs"))?;

    let mut problems = Vec::new();
    for path in walk_rs_files(wifi_src)? {
        let src = std::fs::read_to_string(&path)?;
        for id_const in extract_send_api_req_ids(&src) {
            // An unknown ident is a non-literal argument (a variable, a cast) or
            // a constant declared elsewhere — not something to guess at.
            let Some(&id) = rust_ids.get(&id_const) else {
                continue;
            };
            if !supported.contains_key(&id) {
                problems.push(format!("{id_const} (0x{id:04x}) in {}", path.display()));
            }
        }
    }

    if problems.is_empty() {
        return Ok(());
    }
    problems.sort();
    problems.dedup();
    bail!(
        "{} send_api_req call(s) use an api_id the pinned NP image does not dispatch:\n  {}\n\n\
         The prebuilt NP image does not implement these APIs; drop the call (supported ids are \
         listed in {}).",
        problems.len(),
        problems.join("\n  "),
        list.display()
    )
}

/// `0xNNNN WHC_API_NAME` lines from `np_api_ids.txt`; `#` starts a comment.
fn parse_np_api_ids(path: &Path) -> Result<std::collections::HashMap<u32, String>> {
    let src =
        std::fs::read_to_string(path).with_context(|| format!("cannot read {}", path.display()))?;
    let mut out = std::collections::HashMap::new();
    for line in src.lines() {
        let line = line.split('#').next().unwrap_or(line).trim();
        if line.is_empty() {
            continue;
        }
        let Some((value, name)) = line.split_once(char::is_whitespace) else {
            continue;
        };
        let Some(id) = value
            .trim()
            .strip_prefix("0x")
            .and_then(|h| u32::from_str_radix(h, 16).ok())
        else {
            continue;
        };
        out.insert(id, name.trim().to_string());
    }
    if out.is_empty() {
        bail!("{} parsed to zero api_ids", path.display());
    }
    Ok(out)
}

/// api_id idents from bare `send_api_req(API_X, ...)` call sites.
fn extract_send_api_req_ids(src: &str) -> Vec<String> {
    let mut out = Vec::new();
    for (i, _) in src.match_indices("send_api_req(") {
        let rest = &src[i + "send_api_req(".len()..];
        let Some(arg) = rest.split(',').next() else {
            continue;
        };
        let arg = arg.trim();
        if !arg.is_empty() && arg.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            out.push(arg.to_string());
        }
    }
    out
}

/// `pub const API_X: u32 = 0xNNNN;` from `whc/msg.rs`.
fn parse_rust_api_ids(msg_rs: &Path) -> Result<std::collections::HashMap<String, u32>> {
    let src = std::fs::read_to_string(msg_rs)
        .with_context(|| format!("cannot read {}", msg_rs.display()))?;
    let mut out = std::collections::HashMap::new();
    for line in src.lines() {
        let line = line.split("//").next().unwrap_or(line).trim();
        let Some(rest) = line.strip_prefix("pub const ") else {
            continue;
        };
        let Some((name, value)) = rest.split_once(':') else {
            continue;
        };
        let Some((_, value)) = value.split_once('=') else {
            continue;
        };
        let value = value.trim().trim_end_matches(';').trim();
        let parsed = value
            .strip_prefix("0x")
            .and_then(|h| u32::from_str_radix(h, 16).ok())
            .or_else(|| value.parse::<u32>().ok());
        if let Some(v) = parsed {
            out.insert(name.trim().to_string(), v);
        }
    }
    Ok(out)
}

/// Every `.rs` file under `dir`, recursively.
fn walk_rs_files(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        for e in std::fs::read_dir(&d)? {
            let p = e?.path();
            if p.is_dir() {
                stack.push(p);
            } else if p.extension().is_some_and(|x| x == "rs") {
                out.push(p);
            }
        }
    }
    Ok(out)
}

/// Byte-compare two files; a missing `b` counts as different.
fn files_equal(a: &Path, b: &Path) -> Result<bool> {
    if !b.is_file() {
        return Ok(false);
    }
    let (ma, mb) = (std::fs::metadata(a)?, std::fs::metadata(b)?);
    if ma.len() != mb.len() {
        return Ok(false);
    }
    Ok(std::fs::read(a)? == std::fs::read(b)?)
}

fn build_rust_binary(bin_name: &str, crate_dir: &Path, build_dir: &Path) -> Result<PathBuf> {
    let cargo_target_dir = build_dir.join("cargo");

    run(Command::new("cargo")
        .args(["build", "--release", "--target", RUST_TARGET])
        .env("CARGO_TARGET_DIR", &cargo_target_dir)
        .current_dir(crate_dir))?;

    let elf = cargo_target_dir
        .join(RUST_TARGET)
        .join("release")
        .join(bin_name);
    if !elf.is_file() {
        bail!("ELF not found after cargo build: {}", elf.display());
    }
    Ok(elf)
}

// ── run example ──────────────────────────────────────────────────────────────

/// Build an example, flash it over UART (`ameba-flash`), then monitor
/// serial. `boot.bin` and the floader come from `ameba_flash::prebuilt`.
fn run_example(name: &str, port: Option<&str>, baud: u32) -> Result<()> {
    let root = repo_root();
    let crate_dir = root.join("examples").join(name);
    let bin_name = name.replace('/', "-");
    build_firmware(&bin_name, &crate_dir)?;

    let serial_port = port
        .map(|s| s.to_string())
        .or_else(|| std::env::var("RTK_SERIAL_PORT").ok())
        .context(
            "serial port required: pass --port <port> or set RTK_SERIAL_PORT environment variable",
        )?;

    let build_dir = root.join("build").join("rtl8721f");
    let app_bin_path = build_dir.join("app.bin");
    let app_bin = std::fs::read(&app_bin_path)
        .with_context(|| format!("failed to read {}", app_bin_path.display()))?;

    println!("[run] flashing on {serial_port} ...");
    flash_session(
        &serial_port,
        prebuilt::FLOADER_BIN,
        prebuilt::BOOT_BIN,
        &app_bin,
    )?;
    println!("[run] flash done.");

    monitor_serial(&serial_port, baud)?;
    Ok(())
}

// ── check / lint / fmt / clean / ci ──────────────────────────────────────────

/// Restrict `targets` to `packages` (all of them if `packages` is empty).
///
/// Errors on any label with no match, so a typo fails loudly instead of
/// silently checking/linting/formatting zero crates.
fn filter_targets(targets: Vec<CrateTarget>, packages: &[String]) -> Result<Vec<CrateTarget>> {
    if packages.is_empty() {
        return Ok(targets);
    }
    let wanted: HashSet<&str> = packages.iter().map(String::as_str).collect();
    let mut matched: HashSet<&str> = HashSet::new();
    let mut filtered = Vec::new();
    for t in &targets {
        if wanted.contains(t.label.as_str()) && matched.insert(t.label.as_str()) {
            filtered.push(t.clone());
        }
    }
    if matched.len() < wanted.len() {
        let missing: Vec<&str> = wanted
            .iter()
            .filter(|p| !matched.contains(*p))
            .copied()
            .collect();
        bail!(
            "unknown package(s) '{}' (available: {})",
            missing.join(", "),
            targets
                .iter()
                .map(|t| t.label.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
    Ok(filtered)
}

/// `cargo check` every crate.
///
/// Fail-fast (propagates with `?`). Libraries are checked before examples
/// so a library break is reported once, not 32 times.
fn run_check(packages: &[String]) -> Result<()> {
    let targets = filter_targets(all_crate_targets()?, packages)?;
    println!("[check] {} crates", targets.len());

    for (i, t) in targets.iter().enumerate() {
        println!("[check] ({}/{}) {}", i + 1, targets.len(), t.label);
        cargo_in(t, "check", &[], &[])?;
    }

    println!("\n[done] check passed for {} crates", targets.len());
    Ok(())
}

/// `cargo clippy` every crate.
///
/// Fail-fast per cmd-lint spec ("abort immediately on any step failure").
///
/// `--fix` and `-D warnings` are mutually exclusive branches: in fix mode
/// clippy rewrites source, so denying warnings would abort the very run
/// that is meant to repair them. `--fix` also needs `--allow-dirty` because
/// this tree is normally dirty while developing.
///
/// `--no-deps` keeps clippy from linting path dependencies: without it,
/// linting an example re-lints `ameba-hal` and attributes its warnings to
/// the example.
fn run_lint(fix: bool, packages: &[String]) -> Result<()> {
    let targets = filter_targets(all_crate_targets()?, packages)?;
    println!("[lint] {} crates (fix={})", targets.len(), fix);

    for (i, t) in targets.iter().enumerate() {
        println!("[lint] ({}/{}) {}", i + 1, targets.len(), t.label);
        if fix {
            cargo_in(
                t,
                "clippy",
                &["--fix", "--allow-dirty", "--allow-staged"],
                &[],
            )?;
        } else {
            cargo_in(t, "clippy", &["--no-deps"], &["-D", "warnings"])?;
        }
    }

    println!("\n[done] lint passed for {} crates", targets.len());
    Ok(())
}

/// `cargo fmt` every crate. Fail-fast.
///
/// rustfmt is a source-only pass: no `--target` needed, so every crate is
/// invoked as a host crate.
fn run_fmt(check_only: bool, packages: &[String]) -> Result<()> {
    let mut targets = filter_targets(all_crate_targets()?, packages)?;
    targets.retain(|t| !t.skip_fmt);
    for t in &mut targets {
        t.host = true;
    }
    println!("[fmt] {} crates (check={})", targets.len(), check_only);

    let pre: &[&str] = if check_only { &["--check"] } else { &[] };
    for (i, t) in targets.iter().enumerate() {
        println!("[fmt] ({}/{}) {}", i + 1, targets.len(), t.label);
        cargo_in(t, "fmt", pre, &[])?;
    }

    println!("\n[done] fmt passed for {} crates", targets.len());
    Ok(())
}

/// Remove build artifacts and report reclaimed space.
///
/// Removes directories directly instead of shelling out to `cargo clean`
/// per package: `build/rtl8721f/` holds non-cargo packaging outputs
/// (`app.bin`, `km4tz_image2_all.bin`, staged `boot.bin`) that `cargo clean`
/// does not know about, and the cmd-clean spec asks for a reclaimed-size
/// report, which `cargo clean` cannot provide.
fn run_clean(all: bool) -> Result<()> {
    let root = repo_root();
    let mut reclaimed = 0u64;

    // build/ holds both packaging outputs and the cargo target dir
    // (build/rtl8721f/cargo) used by build_firmware.
    reclaimed += remove_dir_reporting(&root.join("build"))?;

    if all {
        reclaimed += remove_dir_reporting(&root.join("target"))?;

        // Crate-local target/ dirs, created when building a crate by hand.
        for t in all_crate_targets()? {
            reclaimed += remove_dir_reporting(&t.dir.join("target"))?;
        }
    }

    println!(
        "[done] clean complete, reclaimed ~{}",
        human_bytes(reclaimed)
    );
    Ok(())
}

/// Remove `dir` if present, returning the bytes reclaimed (0 if absent).
fn remove_dir_reporting(dir: &Path) -> Result<u64> {
    if !dir.exists() {
        return Ok(0);
    }
    let size = dir_size(dir);
    std::fs::remove_dir_all(dir).with_context(|| format!("failed to remove {}", dir.display()))?;
    println!("[clean] removed {} (~{})", dir.display(), human_bytes(size));
    Ok(size)
}

fn dir_size(path: &Path) -> u64 {
    let mut total = 0u64;
    if let Ok(rd) = std::fs::read_dir(path) {
        for entry in rd.filter_map(|e| e.ok()) {
            let p = entry.path();
            if p.is_file() {
                total += std::fs::metadata(&p).map(|m| m.len()).unwrap_or(0);
            } else if p.is_dir() {
                total += dir_size(&p);
            }
        }
    }
    total
}

fn human_bytes(bytes: u64) -> String {
    if bytes >= 1 << 30 {
        format!("{:.1} GiB", bytes as f64 / (1u64 << 30) as f64)
    } else if bytes >= 1 << 20 {
        format!("{:.1} MiB", bytes as f64 / (1u64 << 20) as f64)
    } else if bytes >= 1 << 10 {
        format!("{:.1} KiB", bytes as f64 / (1u64 << 10) as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// Generate rustdoc for the firmware library crates into `docs/api/`.
///
/// Examples are excluded: they are `#[no_main]` binaries with no public API.
///
/// All crates share ONE cargo target dir under `build/` so rustdoc merges
/// them into a single cross-linked `doc/` tree, which is then copied to
/// `docs/api/`. `CARGO_TARGET_DIR` must NOT point at `docs/` — cargo would
/// write every intermediate object there (hundreds of MB inside the repo),
/// and the HTML would land in `docs/api/<triple>/doc/` rather than
/// `docs/api/`.
///
/// `RUSTDOCFLAGS=-D warnings` makes rustdoc diagnostics fail the build. Without
/// it this command printed warnings and still exited 0, so `ci` stayed green
/// while 32 broken intra-doc links accumulated unnoticed (rustdoc is the only
/// tool that resolves `[..]` links — `check`, `build` and `clippy` never do).
fn build_doc() -> Result<()> {
    let root = repo_root();
    let target_dir = root.join("build").join("rtl8721f").join("doc-target");
    let out_dir = root.join("docs").join("api");

    let targets: Vec<CrateTarget> = existing_firmware_crate_targets(&root);

    println!("[doc] {} library crates", targets.len());
    for (i, t) in targets.iter().enumerate() {
        println!("[doc] ({}/{}) {}", i + 1, targets.len(), t.label);
        let mut cmd = Command::new("cargo");
        cmd.args(["doc", "--no-deps", "--target", RUST_TARGET])
            .env("CARGO_TARGET_DIR", &target_dir)
            .env("RUSTDOCFLAGS", "-D warnings")
            .current_dir(&t.dir);
        run(&mut cmd).with_context(|| format!("`cargo doc` failed in {}", t.label))?;
    }

    // Host crate: lint-only (own target dir, output not published).
    println!("[doc] ameba-flash (host, lint only)");
    let mut cmd = Command::new("cargo");
    cmd.args(["doc", "--no-deps"])
        .env(
            "CARGO_TARGET_DIR",
            root.join("build").join("host-doc-target"),
        )
        .env("RUSTDOCFLAGS", "-D warnings")
        .current_dir(root.join("ameba-flash"));
    run(&mut cmd).context("`cargo doc` failed in ameba-flash")?;

    // rustdoc HTML lands in <target>/<triple>/doc
    let html = target_dir.join(RUST_TARGET).join("doc");
    if !html.is_dir() {
        bail!("rustdoc output not found: {}", html.display());
    }

    // Replace docs/api wholesale so removed items don't linger.
    if out_dir.exists() {
        std::fs::remove_dir_all(&out_dir)
            .with_context(|| format!("failed to clear {}", out_dir.display()))?;
    }
    std::fs::create_dir_all(&out_dir)?;
    let copied = copy_dir_all(&html, &out_dir)?;

    // Six separate `cargo doc` invocations produce per-crate trees but no
    // shared landing page, so `docs/api/` needs its own generated entry point.
    let entries = write_docs_index(&out_dir)?;

    println!(
        "\n[done] docs → {} ({} files, {} crates indexed)",
        out_dir.display(),
        copied,
        entries
    );
    Ok(())
}

/// Rustdoc support directories that are not documented crates.
const RUSTDOC_INTERNAL_DIRS: &[&str] = &[
    "src",
    "static.files",
    "trait.impl",
    "type.impl",
    "search.index",
    "implementors",
];

/// Write `docs/api/index.html` linking every documented crate.
///
/// The list is discovered from the generated tree rather than from the crate
/// targets, because a bin-only crate such as `hil-test` produces one directory
/// per binary (`gpio/`, `storage/`, ...) instead of a crate directory.
/// Scanning avoids dead links either way.
fn write_docs_index(out_dir: &Path) -> Result<usize> {
    let mut crates: Vec<String> = Vec::new();

    for entry in std::fs::read_dir(out_dir)
        .with_context(|| format!("failed to read {}", out_dir.display()))?
    {
        let entry = entry?;
        if !entry.path().is_dir() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().to_string();
        if RUSTDOC_INTERNAL_DIRS.contains(&name.as_str()) {
            continue;
        }
        if entry.path().join("index.html").is_file() {
            crates.push(name);
        }
    }
    crates.sort();

    let mut html = String::from(
        "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n\
         <title>RTL8721F Rust SDK — API documentation</title>\n\
         <style>body{font-family:sans-serif;max-width:40rem;margin:3rem auto;padding:0 1rem}\
         li{margin:.35rem 0}code{background:#f4f4f4;padding:.1rem .3rem;border-radius:3px}</style>\n\
         </head>\n<body>\n<h1>RTL8721F Rust SDK</h1>\n\
         <p>Generated by <code>cargo xtask build doc</code>.</p>\n<ul>\n",
    );
    for c in &crates {
        html.push_str(&format!("<li><a href=\"{c}/index.html\">{c}</a></li>\n"));
    }
    html.push_str("</ul>\n</body>\n</html>\n");

    let path = out_dir.join("index.html");
    std::fs::write(&path, html).with_context(|| format!("failed to write {}", path.display()))?;
    Ok(crates.len())
}

/// Recursively copy `src` into `dst`, returning the number of files copied.
fn copy_dir_all(src: &Path, dst: &Path) -> Result<usize> {
    let mut count = 0;
    std::fs::create_dir_all(dst)?;
    for entry in
        std::fs::read_dir(src).with_context(|| format!("failed to read {}", src.display()))?
    {
        let entry = entry?;
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if from.is_dir() {
            count += copy_dir_all(&from, &to)?;
        } else {
            std::fs::copy(&from, &to)
                .with_context(|| format!("failed to copy {}", from.display()))?;
            count += 1;
        }
    }
    Ok(count)
}

/// CI step runner: collects failures instead of failing fast, so one broken
/// step does not hide the state of the others, and reports a summary at the
/// end via `run()` / `finish()`, including GitHub Actions `::group::` log
/// folding.
struct Runner {
    failed: Vec<&'static str>,
    started_at: Instant,
    skip: Vec<String>,
    only: Vec<String>,
    executed: usize,
}

impl Runner {
    fn new(steps: Vec<String>, no_lint: bool, no_docs: bool) -> Self {
        let mut skip = Vec::new();
        if no_lint {
            skip.push("lint".to_string());
        }
        if no_docs {
            skip.push("docs".to_string());
        }
        Self {
            failed: Vec::new(),
            started_at: Instant::now(),
            skip,
            only: steps,
            executed: 0,
        }
    }

    fn run(&mut self, id: &str, group: &'static str, op: impl FnOnce() -> Result<()>) {
        if self.skip.iter().any(|s| s == id) {
            println!("[ci] {group}: skipped by request");
            return;
        }
        if !self.only.is_empty() && !self.only.iter().any(|s| s == id) {
            return;
        }
        self.executed += 1;

        println!("::group::{group}");
        match op() {
            Ok(()) => println!("[ci] {group}: OK"),
            Err(e) => {
                eprintln!("[ci] {group}: FAILED: {e:?}");
                self.failed.push(group);
            }
        }
        println!("::endgroup::");
    }

    fn finish(self) -> Result<()> {
        // `only` is validated before the run, so reaching zero here means every
        // requested step was also excluded by --no-lint / --no-docs.
        if self.executed == 0 {
            bail!(
                "no CI steps executed: --steps {:?} were all excluded by {:?}",
                self.only,
                self.skip
            );
        }

        println!(
            "\n[ci] {} step(s) in {:?}",
            self.executed,
            self.started_at.elapsed()
        );
        if self.failed.is_empty() {
            println!("[done] CI passed");
            Ok(())
        } else {
            for f in &self.failed {
                println!("[ci] FAILED: {f}");
            }
            bail!("CI failed: {}", self.failed.join(", "))
        }
    }
}

fn run_ci(steps: Vec<String>, no_lint: bool, no_docs: bool) -> Result<()> {
    // Reject unknown ids before running anything: a partially misspelled
    // --steps must not quietly run the recognised subset.
    let unknown: Vec<&str> = steps
        .iter()
        .filter(|s| !CI_STEPS.contains(&s.as_str()))
        .map(|s| s.as_str())
        .collect();
    if !unknown.is_empty() {
        bail!(
            "unknown --steps value(s): {}\nvalid steps: {}",
            unknown.join(", "),
            CI_STEPS.join(", ")
        );
    }

    let mut runner = Runner::new(steps, no_lint, no_docs);

    runner.run("check", "Check crates", || run_check(&[]));
    runner.run("lint", "Lint crates", || run_lint(false, &[]));
    runner.run("fmt", "Format check", || run_fmt(true, &[]));
    runner.run("docs", "Build documentation", build_doc);

    runner.run("tests", "Build hil-test binaries", || {
        let crate_dir = repo_root().join("hil-test");
        let bins: Vec<(String, PathBuf)> = discover_hil_bins()?
            .into_iter()
            .map(|b| (b, crate_dir.clone()))
            .collect();
        build_batch("hil-test bin", &bins)
    });

    // Catches a PAC edited by hand instead of through svd/. Needs the three
    // pinned generators, so a machine without them reports a skip rather
    // than a failure. Likewise when `svd/` is absent (not shipped in some
    // distributions of this repo, same as `hil-test/`).
    runner.run("pac", "PAC up to date with svd/", || {
        if !repo_root()
            .join("ameba-hal")
            .join("pac")
            .join("rtl8721f")
            .join("svd")
            .is_dir()
        {
            println!("[ci] SKIP PAC check: svd/ not present");
            return Ok(());
        }
        if let Err(e) = check_pac_tool_versions() {
            println!("[ci] SKIP PAC check: {e}");
            return Ok(());
        }
        run_gen_pac(true)
    });

    runner.finish()
}

// ── gen-pac ──────────────────────────────────────────────────────────────────

/// Pinned generator versions. The generated code is only reproducible against
/// an exact toolchain: a patch-level bump in any of these rewrites parts of
/// `lib.rs`, at which point "diff is empty" stops meaning anything. Bumping a
/// version here is a deliberate act that comes with regenerating the PAC.
const PAC_TOOLS: &[(&str, &str)] = &[
    ("svdtools", "0.5.0"),
    ("svd2rust", "0.37.1"),
    ("form", "0.13.0"),
];

/// rustfmt shapes the generated tree just as much as the three tools above --
/// it formats every per-register module under `src/<periph>/`. Unlike them it
/// ships with the toolchain, so it is checked separately and reported with a
/// rustup-flavoured hint.
const PAC_RUSTFMT_VERSION: &str = "1.9.0-stable";

fn run_gen_pac(check: bool) -> Result<()> {
    let root = repo_root();
    let pac = root.join("ameba-hal").join("pac").join("rtl8721f");
    let svd_dir = pac.join("svd");
    let base_svd = svd_dir.join("rtl8721f.base.svd");
    let patch_yaml = svd_dir.join("rtl8721f.yaml");

    for f in [&base_svd, &patch_yaml] {
        if !f.is_file() {
            bail!("missing pipeline input: {}", f.display());
        }
    }

    check_pac_tool_versions()?;
    check_base_svd_shape(&base_svd)?;

    // Work in a scratch dir: svdtools writes `<input>.patched` next to its
    // input, and svd2rust/form only write into the cwd.
    let work = root.join("build").join("rtl8721f").join("pac-gen");
    if work.exists() {
        std::fs::remove_dir_all(&work)
            .with_context(|| format!("failed to clear {}", work.display()))?;
    }
    std::fs::create_dir_all(&work)
        .with_context(|| format!("failed to create {}", work.display()))?;

    let work_base = work.join("rtl8721f.base.svd");
    let work_yaml = work.join("rtl8721f.yaml");
    std::fs::copy(&base_svd, &work_base)?;
    std::fs::copy(&patch_yaml, &work_yaml)?;

    println!("[gen-pac] 1/4 svdtools patch");
    run(Command::new("svdtools")
        .arg("patch")
        .arg("rtl8721f.yaml")
        .current_dir(&work))?;
    let patched = work.join("rtl8721f.base.svd.patched");
    if !patched.is_file() {
        bail!("svdtools produced no {}", patched.display());
    }

    println!("[gen-pac] 2/4 svd2rust");
    run(Command::new("svd2rust")
        .arg("-i")
        .arg(&patched)
        .args(["--target", "cortex-m"])
        .current_dir(&work))?;

    println!("[gen-pac] fixing up svd2rust output for edition 2024 + empty docs");
    fix_pac_lib_rs(&work.join("lib.rs"))?;

    println!("[gen-pac] 3/4 form");
    let src_gen = work.join("src");
    run(Command::new("form")
        .arg("--input")
        .arg(work.join("lib.rs"))
        .arg("--outdir")
        .arg(&src_gen)
        .current_dir(&work))?;

    // `form` always re-emits the crate root as `src/lib.rs` too, but the
    // crate's real root is `lib.rs` at the PAC's top level (`[lib] path =
    // "lib.rs"` in Cargo.toml) -- this copy is dead weight that would
    // otherwise ship as a confusing duplicate.
    let src_gen_lib = src_gen.join("lib.rs");
    if src_gen_lib.is_file() {
        std::fs::remove_file(&src_gen_lib)
            .with_context(|| format!("failed to remove {}", src_gen_lib.display()))?;
    }

    println!("[gen-pac] 4/4 rustfmt");
    rustfmt_register_modules(&src_gen)?;

    let gen_lib = work.join("lib.rs");
    let gen_device_x = work.join("device.x");
    for f in [&gen_lib, &gen_device_x] {
        if !f.is_file() {
            bail!("generator produced no {}", f.display());
        }
    }

    if check {
        let mut stale = Vec::new();
        if !files_equal(&gen_lib, &pac.join("lib.rs"))? {
            stale.push("lib.rs");
        }
        if !files_equal(&gen_device_x, &pac.join("device.x"))? {
            stale.push("device.x");
        }
        if !dirs_equal(&src_gen, &pac.join("src"))? {
            stale.push("src/");
        }
        if !stale.is_empty() {
            bail!(
                "committed PAC is out of date with svd/: {}\n\
                 run `cargo xtask gen-pac` and commit the result\n\
                 (generated tree kept at {} for inspection)",
                stale.join(", "),
                work.display()
            );
        }
        println!("[gen-pac] --check: lib.rs, device.x and src/ all match svd/");
        return Ok(());
    }

    std::fs::copy(&gen_lib, pac.join("lib.rs"))?;
    std::fs::copy(&gen_device_x, pac.join("device.x"))?;
    let dst_src = pac.join("src");
    if dst_src.exists() {
        std::fs::remove_dir_all(&dst_src)
            .with_context(|| format!("failed to clear {}", dst_src.display()))?;
    }
    let n = copy_dir_all(&src_gen, &dst_src)?;
    println!("[gen-pac] wrote lib.rs, device.x and {n} files under src/");
    Ok(())
}

/// Patch svd2rust's raw output to be edition-2024-clean, before `form` splits
/// it into per-peripheral files (so the fixes propagate into those too).
///
/// svd2rust (pinned at the version in [`PAC_TOOLS`]) emits a few constructs
/// edition 2024 either rejects outright or flags under
/// `rust_2024_compatibility`, plus ~2100 doc comments empty after `rustdoc`
/// strips a leading blank line (`clippy::empty_docs`):
///   - a bare `extern "C" { ... }` block of IRQ externs -> needs `unsafe`
///   - `#[no_mangle]` / `#[link_section]` -> need the `#[unsafe(...)]` form
///   - `Peripherals::steal()`'s body performs raw `Xxx::steal()` calls and a
///     `static mut` write and must be wrapped in `unsafe { ... }` (the
///     *other* `steal()`, on the generic `Periph<RB, A>` type, has no unsafe
///     operations in its body and must be left alone)
///   - every `#[doc = ""]` left by a blank doc-comment line
fn fix_pac_lib_rs(lib_rs: &Path) -> Result<()> {
    let mut src = std::fs::read_to_string(lib_rs)
        .with_context(|| format!("failed to read {}", lib_rs.display()))?;

    let empty_docs = src.matches("#[doc = \"\"]").count();
    src = src
        .replace("#[doc = \"\"] ", "")
        .replace("#[doc = \"\"]\n", "");

    let extern_c = "#[cfg (feature = \"rt\")] extern \"C\" {";
    let n = src.matches(extern_c).count();
    if n != 1 {
        bail!("expected exactly 1 bare `extern \"C\"` IRQ block in svd2rust output, found {n}");
    }
    src = src.replace(extern_c, "#[cfg (feature = \"rt\")] unsafe extern \"C\" {");

    let link_section = "#[link_section = \".vector_table.interrupts\"]";
    let n = src.matches(link_section).count();
    if n != 1 {
        bail!("expected exactly 1 `#[link_section]` in svd2rust output, found {n}");
    }
    src = src.replace(
        link_section,
        "#[unsafe(link_section = \".vector_table.interrupts\")]",
    );

    let n = src.matches("#[no_mangle]").count();
    if n != 2 {
        bail!("expected exactly 2 `#[no_mangle]` in svd2rust output, found {n}");
    }
    src = src.replace("#[no_mangle]", "#[unsafe(no_mangle)]");

    let steal_marker = "pub unsafe fn steal () -> Self { DEVICE_PERIPHERALS = true ; Peripherals {";
    let n = src.matches(steal_marker).count();
    if n != 1 {
        bail!("expected exactly 1 `Peripherals::steal()` body in svd2rust output, found {n}");
    }
    let start = src.find(steal_marker).unwrap();
    let body_open = src[start..].find('{').unwrap() + start;
    let mut depth = 0usize;
    let mut body_close = body_open;
    for (i, b) in src.as_bytes()[body_open..].iter().enumerate() {
        match b {
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    body_close = body_open + i;
                    break;
                }
            }
            _ => {}
        }
    }
    if body_close == body_open {
        bail!("failed to find matching close brace for `Peripherals::steal()` body");
    }
    let inner = &src[body_open + 1..body_close];
    let wrapped = format!("{{ unsafe {{ {inner} }} }}");
    src = format!("{}{}{}", &src[..body_open], wrapped, &src[body_close + 1..]);

    println!(
        "[gen-pac] fixup: stripped {empty_docs} empty doc attrs, \
         1 extern \"C\" block, 1 link_section, 2 no_mangle, 1 steal() body wrap"
    );

    std::fs::write(lib_rs, src).with_context(|| format!("failed to write {}", lib_rs.display()))
}

/// Reject a generator whose version is not the one the committed PAC was built
/// with. `--version` output is `"<name> <semver>"`, with svd2rust appending a
/// build-metadata field that is empty in release builds.
fn check_pac_tool_versions() -> Result<()> {
    for (tool, want) in PAC_TOOLS {
        let out = Command::new(tool)
            .arg("--version")
            .output()
            .with_context(|| format!("{tool} not found on PATH -- cargo install {tool}"))?;
        if !out.status.success() {
            bail!("{tool} --version failed");
        }
        let text = String::from_utf8_lossy(&out.stdout);
        let got = text
            .split_whitespace()
            .nth(1)
            .with_context(|| format!("cannot parse {tool} version from {text:?}"))?;
        if got != *want {
            bail!(
                "{tool} {got} found, but the committed PAC was generated with \
                 {want}; a different version produces different code.\n\
                 install the pinned version: cargo install {tool} --version {want} --force"
            );
        }
        println!("[gen-pac] {tool} {want}");
    }

    let out = Command::new("rustfmt")
        .arg("--version")
        .output()
        .context("rustfmt not found -- rustup component add rustfmt")?;
    if !out.status.success() {
        bail!("rustfmt --version failed");
    }
    let text = String::from_utf8_lossy(&out.stdout);
    let got = text
        .split_whitespace()
        .nth(1)
        .with_context(|| format!("cannot parse rustfmt version from {text:?}"))?;
    if got != PAC_RUSTFMT_VERSION {
        bail!(
            "rustfmt {got} found, but the committed PAC was formatted with \
             {PAC_RUSTFMT_VERSION}; a different version reformats every \
             per-register module.\n\
             rustfmt ships with the toolchain, so either pin the toolchain that \
             carries {PAC_RUSTFMT_VERSION} or regenerate the PAC with this \
             rustfmt and update PAC_RUSTFMT_VERSION in xtask."
        );
    }
    println!("[gen-pac] rustfmt {PAC_RUSTFMT_VERSION}");
    Ok(())
}

/// Two conditions the patch layer depends on that would otherwise fail in a
/// way that does not point at the base SVD. Everything else about the base SVD
/// is covered by `--check`, which diffs the whole generated tree.
fn check_base_svd_shape(base_svd: &Path) -> Result<()> {
    let svd = std::fs::read_to_string(base_svd)
        .with_context(|| format!("failed to read {}", base_svd.display()))?;

    // A <cpu> here would mean the patch layer's `_modify: cpu:` silently has
    // no effect, and the real CPU description is whatever the base SVD says.
    if svd.contains("<cpu>") {
        bail!(
            "{}: has a <cpu> element, but the CPU description belongs to the \
             patch layer (the workbook has no CPU data to generate it from)",
            base_svd.display()
        );
    }

    // svd-parser rejects `0x4080_a000` with "invalid digit found in string",
    // which does not say which element or why. Name the real problem instead.
    for tag in [
        "baseAddress",
        "addressOffset",
        "size",
        "resetValue",
        "resetMask",
        "offset",
    ] {
        let open = format!("<{tag}>");
        for (i, chunk) in svd.split(&open).enumerate().skip(1) {
            let Some(value) = chunk.split('<').next() else {
                continue;
            };
            if value.contains('_') {
                bail!(
                    "{}: <{tag}> #{i} is `{value}` -- digit-group separators are \
                     not valid SVD; strip the underscore",
                    base_svd.display()
                );
            }
        }
    }

    Ok(())
}

/// Format the per-register modules `form` wrote under `src/<periph>/`.
///
/// Only files at depth >= 2 are formatted. `src/lib.rs` and the per-peripheral
/// `src/<periph>.rs` files stay exactly as `form` emitted them (one very long
/// line each), matching the committed tree -- running rustfmt over those would
/// rewrite ~92 files that are checked in unformatted.
///
/// rustfmt is invoked on batches of standalone files rather than on the crate
/// root, because entering through `lib.rs` would reformat it too.
fn rustfmt_register_modules(src: &Path) -> Result<()> {
    let mut files = Vec::new();
    for entry in
        std::fs::read_dir(src).with_context(|| format!("failed to read {}", src.display()))?
    {
        let dir = entry?.path();
        if !dir.is_dir() {
            continue;
        }
        collect_rs_files(&dir, &mut files)?;
    }
    if files.is_empty() {
        bail!("{}: form produced no per-register modules", src.display());
    }

    // Keep each command line well inside the platform limit.
    let mut formatted = 0usize;
    for batch in files.chunks(200) {
        let mut cmd = Command::new("rustfmt");
        cmd.args(["--edition", "2024"]).args(batch);
        let status = cmd.status().context("failed to spawn rustfmt")?;
        if !status.success() {
            bail!("rustfmt failed (exit {})", status.code().unwrap_or(-1));
        }
        formatted += batch.len();
    }
    println!("[gen-pac] rustfmt: {formatted} register modules");
    Ok(())
}

fn collect_rs_files(dir: &Path, out: &mut Vec<PathBuf>) -> Result<()> {
    for entry in
        std::fs::read_dir(dir).with_context(|| format!("failed to read {}", dir.display()))?
    {
        let path = entry?.path();
        if path.is_dir() {
            collect_rs_files(&path, out)?;
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
    Ok(())
}

/// Recursively compare two directory trees by content.
fn dirs_equal(a: &Path, b: &Path) -> Result<bool> {
    if !b.is_dir() {
        return Ok(false);
    }
    let list = |dir: &Path| -> Result<Vec<(OsString, bool)>> {
        let mut v = Vec::new();
        for entry in
            std::fs::read_dir(dir).with_context(|| format!("failed to read {}", dir.display()))?
        {
            let entry = entry?;
            v.push((entry.file_name(), entry.path().is_dir()));
        }
        v.sort();
        Ok(v)
    };
    let (ea, eb) = (list(a)?, list(b)?);
    if ea != eb {
        return Ok(false);
    }
    for (name, is_dir) in ea {
        let (pa, pb) = (a.join(&name), b.join(&name));
        let same = if is_dir {
            dirs_equal(&pa, &pb)?
        } else {
            files_equal(&pa, &pb)?
        };
        if !same {
            return Ok(false);
        }
    }
    Ok(true)
}

// ── probe-rs ─────────────────────────────────────────────────────────────────

fn normalize_yaml_path(chip: &Path, root: &Path) -> PathBuf {
    if chip.is_absolute() {
        chip.to_path_buf()
    } else {
        root.join(chip)
    }
}

fn wait_for_halt(core: &mut probe_rs::Core<'_>, timeout: Duration, reason: &str) -> Result<()> {
    let deadline = Instant::now() + timeout;
    loop {
        let status = core
            .status()
            .with_context(|| format!("failed to read core status while waiting for {reason}"))?;
        if status.is_halted() {
            return Ok(());
        }
        if Instant::now() >= deadline {
            bail!("timeout waiting for {reason}");
        }
        std::thread::sleep(Duration::from_millis(2));
    }
}

fn read_halfword(core: &mut probe_rs::Core<'_>, address: u64) -> Result<u16> {
    let mut bytes = [0u8; 2];
    core.read_8(address, &mut bytes)
        .with_context(|| format!("failed to read instruction at 0x{address:08x}"))?;
    Ok(u16::from_le_bytes(bytes))
}

fn run_hil_test_session(
    boot_bin: &Path,
    app_bin: &Path,
    chip_yaml: &Path,
    flash_base: u32,
) -> Result<()> {
    // TODO(hil): `MOVS_ADDR` is a raw instruction address inside the target image,
    // used only to park a HW breakpoint across the mailbox handshake. Any rebuild
    // moves the instruction, so this silently points at unrelated code. Resolve the
    // symbol from the ELF (or drop the breakpoint — the handshake is driven by the
    // mailbox value) before the HIL flow is declared done.
    const MOVS_ADDR: u64 = 0x2000_6A74;
    const BKPT_INSN: u16 = 0xBEAB;
    // TODO(hil): `HIL_ADDR` is a hardcoded SRAM mailbox that is *not* reserved in
    // ameba-hal/ld/rtl8721f/ameba.x, and it is duplicated in
    // hil-test/src/bin/gpio.rs. It sits just past gpio's .bss (~1.4 KB of headroom)
    // but inside the storage binary's .text, so only gpio answers the handshake and
    // every other bin times out below. The HOST_ARMED write is a no-op too: the boot
    // ROM reloads the SRAM image after the reset that follows it. Reserve a section
    // in the linker script and export the address to both sides.
    const HIL_ADDR: u64 = 0x2000_C000;
    const HOST_ARMED: u32 = 0x484F_5354;
    const TARGET_WAITING: u32 = 0xA17E_0001;
    const HIL_MAGIC: u32 = 0x4849_4C21;

    eprintln!("[hil] reading chip YAML: {}", chip_yaml.display());
    let yaml_text = std::fs::read_to_string(chip_yaml)
        .with_context(|| format!("failed to read chip YAML: {}", chip_yaml.display()))?;

    let mut registry = Registry::from_builtin_families();
    registry
        .add_target_family_from_yaml(&yaml_text)
        .context("invalid RTL8721F YAML")?;

    eprintln!("[hil] attaching and flashing ...");
    let mut session = Session::auto_attach_with_registry(
        "RTL8721F",
        probe_rs::SessionConfig {
            permissions: Permissions::default(),
            speed: Some(5000),
            protocol: None,
        },
        &registry,
    )
    .context("failed to attach RTL8721F")?;

    eprintln!("[hil] connected, downloading boot.bin (0x{FLASH_BOOT_BASE:08x}) ...");
    probe_rs::flashing::download_file_with_options(
        &mut session,
        boot_bin,
        probe_rs::flashing::BinLoader(probe_rs::flashing::BinOptions {
            base_address: Some(FLASH_BOOT_BASE as u64),
            skip: 0,
        }),
        probe_rs::flashing::DownloadOptions::default(),
    )
    .with_context(|| format!("failed to download binary: {}", boot_bin.display()))?;

    eprintln!("[hil] downloading app.bin (0x{flash_base:08x}) ...");
    probe_rs::flashing::download_file_with_options(
        &mut session,
        app_bin,
        probe_rs::flashing::BinLoader(probe_rs::flashing::BinOptions {
            base_address: Some(flash_base as u64),
            skip: 0,
        }),
        probe_rs::flashing::DownloadOptions::default(),
    )
    .with_context(|| format!("failed to download binary: {}", app_bin.display()))?;

    eprintln!("[hil] download done.");

    let mut core = session.core(0).context("failed to open core 0")?;
    let pc_id = core
        .registers()
        .pc()
        .context("PC register not available")?
        .id();
    let xpsr_id = core
        .registers()
        .psr()
        .context("xPSR register not available")?
        .id();

    eprintln!("[hil] register IDs: PC={pc_id:?} xPSR={xpsr_id:?}");

    core.write_word_32(HIL_ADDR, HOST_ARMED)
        .context("failed to initialize HIL mailbox")?;
    eprintln!("[hil] mailbox initialized to HOST_ARMED=0x{HOST_ARMED:08x}");

    core.set_hw_breakpoint(MOVS_ADDR)
        .with_context(|| format!("failed to set hardware breakpoint at 0x{MOVS_ADDR:08x}"))?;
    eprintln!("[hil] HW breakpoint set at 0x{MOVS_ADDR:08x}");

    eprintln!("[hil] reset and releasing target ...");
    core.reset()?;
    eprintln!("[hil] target running, polling handshake address ...");

    let handshake_deadline = Instant::now() + Duration::from_secs(30);
    let mut previous_value: Option<u32> = None;

    loop {
        let value = core
            .read_word_32(HIL_ADDR)
            .context("failed to read HIL mailbox")?;
        if previous_value != Some(value) {
            eprintln!("[hil] mailbox 0x{HIL_ADDR:08x} = 0x{value:08x}");
            previous_value = Some(value);
        }

        if value == TARGET_WAITING {
            let status = core
                .status()
                .context("failed to read status before releasing target")?;
            eprintln!(
                "[hil] target waiting; status={status:?}, writing HIL_MAGIC=0x{HIL_MAGIC:08x}"
            );
            core.write_word_32(HIL_ADDR, HIL_MAGIC)
                .context("failed to release target through mailbox")?;
            if status.is_halted() {
                eprintln!("[hil] target unexpectedly halted during handshake; resuming it");
                core.run()
                    .context("failed to resume target during handshake")?;
            }
            break;
        }

        if Instant::now() >= handshake_deadline {
            bail!(
                "timeout: target never published TARGET_WAITING; last mailbox value=0x{value:08x}"
            );
        }
        std::thread::sleep(Duration::from_millis(50));
    }

    core.clear_hw_breakpoint(MOVS_ADDR)
        .context("failed to clear MOVS hardware breakpoint")?;
    eprintln!("[hil] MOVS breakpoint cleared, entering semihosting loop ...");

    loop {
        core.run().context("failed to resume core")?;
        wait_for_halt(&mut core, Duration::from_secs(10), "semihosting")?;

        let bkpt_pc: u32 = core.read_core_reg(pc_id)?;
        let xpsr: u32 = core.read_core_reg(xpsr_id)?;
        let ipsr = xpsr & 0x1FF;

        // probe-rs decodes the semihosting call itself (it verifies the halt is on
        // `BKPT 0xAB` before decoding), so match the typed command rather than
        // pattern-matching its Debug representation.
        let command = match core.status().context("failed to read core status")? {
            CoreStatus::Halted(HaltReason::Breakpoint(BreakpointCause::Semihosting(cmd))) => cmd,
            other => {
                let insn = read_halfword(&mut core, bkpt_pc as u64)?;
                eprintln!(
                    "[hil] unexpected halt: PC=0x{bkpt_pc:08x} insn=0x{insn:04x} \
                     IPSR={ipsr} status={other:?}"
                );
                if ipsr != 0 {
                    bail!("target hardfault at PC=0x{bkpt_pc:08x} (IPSR={ipsr})");
                }
                if insn != BKPT_INSN {
                    bail!("unexpected halt: not at semihosting BKPT");
                }
                bail!("halted at BKPT but probe-rs did not decode a semihosting call");
            }
        };
        if ipsr != 0 {
            bail!("semihosting BKPT taken in exception context (IPSR={ipsr})");
        }

        match command {
            // embedded-test signals "all tests passed" with SYS_EXIT / ExitSuccess.
            SemihostingCommand::ExitSuccess => {
                eprintln!("\n[hil] target reported success");
                return Ok(());
            }
            // ...and a failing test with an exit reason/status. Propagate it as an
            // error so `cargo xtask run-hil-test` / `flash-run` exit non-zero.
            SemihostingCommand::ExitError(details) => {
                eprintln!("\n[hil] target reported failure: {details}");
                bail!("HIL test failed on target: {details}");
            }
            // Covers both SYS_WRITEC (single char) and SYS_WRITE0 (zero-terminated
            // string); `read` handles both, so the whole string is printed.
            SemihostingCommand::WriteConsole(request) => {
                let text = request
                    .read(&mut core)
                    .context("failed to read console string")?;
                eprint!("{text}");
            }
            // embedded-test reads the command line to receive the test filter.
            SemihostingCommand::GetCommandLine(request) => {
                request
                    .write_command_line_to_target(&mut core, "")
                    .context("failed to answer SYS_GET_CMDLINE")?;
            }
            SemihostingCommand::Unknown(details) => {
                eprintln!(
                    "[hil] unhandled semihosting op=0x{:04x} param=0x{:08x}",
                    details.operation, details.parameter
                );
                details
                    .write_status(&mut core, -1)
                    .context("failed to report unsupported semihosting op")?;
            }
            other => {
                bail!("unsupported semihosting request: {other:?}");
            }
        }

        // probe-rs does not advance past the BKPT; the host must do it.
        core.write_core_reg(pc_id, bkpt_pc + 2)?;
    }
}
