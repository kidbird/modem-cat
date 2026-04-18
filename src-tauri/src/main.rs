#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    std::env::set_var("NO_PROXY", "tauri.localhost,localhost,127.0.0.1");
    modem_cat_lib::run()
}
