#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

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
