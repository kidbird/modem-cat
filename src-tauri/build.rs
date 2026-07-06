use std::fs;
use std::path::{Path, PathBuf};

fn has_files(path: &Path) -> bool {
    path.exists()
        && fs::read_dir(path)
            .map(|mut entries| entries.any(|entry| entry.is_ok()))
            .unwrap_or(false)
}

fn copy_dir_recursive(from: &Path, to: &Path) -> std::io::Result<()> {
    fs::create_dir_all(to)?;
    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let dest = to.join(entry.file_name());
        if file_type.is_dir() {
            copy_dir_recursive(&entry.path(), &dest)?;
        } else {
            fs::copy(entry.path(), dest)?;
        }
    }
    Ok(())
}

fn ensure_fixed_runtime(manifest_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let target = std::env::var("TARGET").unwrap_or_default();
    if !target.contains("windows") {
        return Ok(());
    }

    let staged_runtime = manifest_dir.join("webview2-runtime");
    if has_files(&staged_runtime) {
        return Ok(());
    }

    let legacy_runtime = manifest_dir.join("../webview2-runtime");
    if has_files(&legacy_runtime) {
        if staged_runtime.exists() {
            fs::remove_dir_all(&staged_runtime)?;
        }
        copy_dir_recursive(&legacy_runtime, &staged_runtime)?;
        println!(
            "cargo:warning=staged fixed WebView2 runtime from legacy root cache -> {:?}",
            staged_runtime
        );
        return Ok(());
    }

    Err(format!(
        "fixed WebView2 runtime is missing. Expected {:?} or legacy cache {:?}. Run scripts/setup-webview2.ps1 first.",
        staged_runtime, legacy_runtime
    )
    .into())
}

fn main() {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let frontend = manifest_dir.join("../src/desktop");
    println!(
        "cargo:warning=MANIFEST_DIR={}, frontendDist={frontend:?}",
        manifest_dir.display()
    );
    println!("cargo:rerun-if-changed=../src/desktop/index.html");
    ensure_fixed_runtime(&manifest_dir).expect("failed to prepare fixed WebView2 runtime");
    tauri_build::build()
}
