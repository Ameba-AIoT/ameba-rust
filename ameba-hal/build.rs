fn main() {
    let dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search={dir}/ld/rtl8721f");
    println!("cargo:rerun-if-changed=ld/rtl8721f/ameba.x");
    println!("cargo:rerun-if-changed=build.rs");
}
