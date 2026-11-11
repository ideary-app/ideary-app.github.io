// Remote pages (the Ideary site) may only call app commands that are listed here
// and allowed in capabilities/default.json.
const COMMANDS: &[&str] = &["app_version", "popout", "set_on_top", "close_window", "install_update", "google_token", "google_login", "google_logout"];

fn main() {
    tauri_build::try_build(tauri_build::Attributes::new().app_manifest(tauri_build::AppManifest::new().commands(COMMANDS)))
        .expect("failed to run tauri-build");
}
