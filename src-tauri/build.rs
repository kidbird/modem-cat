fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let frontend = std::path::Path::new(&manifest_dir).join("../src/desktop");
    println!("cargo:warning=MANIFEST_DIR={manifest_dir}, frontendDist={frontend:?}");
    println!("cargo:rerun-if-changed=../src/desktop/index.html");
    tauri_build::build()
}
