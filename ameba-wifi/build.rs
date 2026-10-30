fn main() {
    let dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    // Add WiFi ROM symbols — only apps linking ameba-wifi will need them on the linker path.
    println!("cargo:rustc-link-search={dir}/ld/rtl8721f");
    println!("cargo:rustc-link-arg=-T");
    println!("cargo:rustc-link-arg=ameba_rom_symbol_wifi.x");
    println!("cargo:rerun-if-changed=ld/rtl8721f/ameba_rom_symbol_wifi.x");
    println!("cargo:rerun-if-changed=build.rs");
}
