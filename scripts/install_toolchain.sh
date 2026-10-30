#!/usr/bin/env bash
# Idempotent rustup + toolchain + targets install for the Ameba Rust SDK.
#
# - Installs rustup to ~/.cargo if absent (does NOT modify shell rc files).
# - Defers the actual toolchain + target download to the first cargo call,
#   driven by ../rust-toolchain.toml (rolling `stable` channel). This keeps the
#   script fast and means per-target downloads happen only when that target is
#   actually built.
#
# Network: requires reachability to sh.rustup.rs and static.rust-lang.org.
# Override RUSTUP_INIT_URL / RUSTUP_DIST_SERVER if your environment uses
# an internal mirror.
set -euo pipefail

CARGO_BIN="${HOME}/.cargo/bin"
RUSTUP_INIT_URL="${RUSTUP_INIT_URL:-https://sh.rustup.rs}"

if ! command -v rustup >/dev/null 2>&1 && [ ! -x "${CARGO_BIN}/rustup" ]; then
    echo "[install_toolchain] rustup not found — bootstrapping from ${RUSTUP_INIT_URL}"
    curl --proto '=https' --tlsv1.2 -sSf "${RUSTUP_INIT_URL}" \
        | sh -s -- -y --default-toolchain none --profile minimal --no-modify-path
else
    echo "[install_toolchain] rustup already present at $(command -v rustup 2>/dev/null || echo "${CARGO_BIN}/rustup")"
fi

# Ensure ${CARGO_BIN} is on PATH for the rest of this script.
export PATH="${CARGO_BIN}:${PATH}"

# Touch the toolchain file so rustup primes the channel + components + targets
# defined in rust-toolchain.toml. Running rustc inside the repo does this.
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
REPO_ROOT="$( cd "${SCRIPT_DIR}/.." >/dev/null 2>&1 && pwd )"
( cd "${REPO_ROOT}" && rustc --version >/dev/null )

echo "[install_toolchain] active toolchain:"
( cd "${REPO_ROOT}" && rustup show active-toolchain )

echo "[install_toolchain] installed targets:"
( cd "${REPO_ROOT}" && rustup target list --installed )

cat <<EOF

[install_toolchain] done. If a new shell can't find cargo, add this once:
    export PATH="\$HOME/.cargo/bin:\$PATH"
or source ~/.cargo/env.
EOF
