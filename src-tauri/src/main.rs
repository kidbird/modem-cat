#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

fn main() {
    // Safety: called before any threads are spawned (Tauri runtime not yet started).
    unsafe {
        std::env::set_var("NO_PROXY", "tauri.localhost,localhost,127.0.0.1");
    }
    modem_cat_lib::run()
}
