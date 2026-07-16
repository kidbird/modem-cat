#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

// WebView2 handling: the app relies solely on the system-installed WebView2
// runtime, discovered by Tauri/wry at runtime. We deliberately do NOT probe the
// registry or attempt to launch a bootstrapper here — doing so caused repeated
// false "WebView2 missing" reports (a stale `webview2-runtime/` dir next to the
// exe makes wry misdetect the runtime). The startup log still records whether
// such a legacy dir is present (see startup_diagnostics.rs) for diagnostics;
// the build pipeline (build.ps1) scrubs it from dist/.
fn main() {
    modem_cat_lib::install_startup_diagnostics();

    // Safety: called before any threads are spawned (Tauri runtime not yet started).
    unsafe {
        std::env::set_var("NO_PROXY", "tauri.localhost,localhost,127.0.0.1");
    }

    if let Err(error) = modem_cat_lib::run() {
        modem_cat_lib::report_startup_error(&format!(
            "startup failed before UI became interactive: {error}"
        ));
        std::process::exit(1);
    }
}
