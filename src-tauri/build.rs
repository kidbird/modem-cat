use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let frontend = manifest_dir.join("../src/desktop");
    println!(
        "cargo:warning=MANIFEST_DIR={}, frontendDist={frontend:?}",
        manifest_dir.display()
    );
    println!("cargo:rerun-if-changed=../src/desktop/index.html");
    tauri_build::build()
}
