// Ideary desktop: a small shell around https://ideary-app.github.io/ that adds what a
// browser can't do: pop-out windows that stay on top, and updating itself.
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

const SITE: &str = "https://ideary-app.github.io/";

#[tauri::command]
fn app_version(app: AppHandle) -> String {
    app.package_info().version.to_string()
}

/// Open (or bring back) a floating window showing one piece of Ideary.
/// `path` is the query string after the site, e.g. "?popout=chat&id=abc".
#[tauri::command]
fn popout(app: AppHandle, label: String, path: String, title: String, width: f64, height: f64, on_top: bool) -> Result<(), String> {
    let label: String = label.chars().map(|c| if c.is_ascii_alphanumeric() || c == '-' || c == '_' { c } else { '-' }).take(60).collect();
    if let Some(w) = app.get_webview_window(&label) {
        let _ = w.set_always_on_top(on_top);
        let _ = w.unminimize();
        let _ = w.set_focus();
        return Ok(());
    }
    if !path.starts_with('?') { return Err("bad path".into()); }
    let url = format!("{}{}", SITE, path);
    WebviewWindowBuilder::new(&app, &label, WebviewUrl::External(url.parse().map_err(|e| format!("{e}"))?))
        .title(title)
        .inner_size(width, height)
        .min_inner_size(260.0, 200.0)
        .always_on_top(on_top)
        .build()
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn set_on_top(app: AppHandle, label: String, on: bool) -> Result<(), String> {
    let w = app.get_webview_window(&label).ok_or("no window")?;
    w.set_always_on_top(on).map_err(|e| e.to_string())
}

#[tauri::command]
fn close_window(app: AppHandle, label: String) -> Result<(), String> {
    let w = app.get_webview_window(&label).ok_or("no window")?;
    w.close().map_err(|e| e.to_string())
}

/// Download the new installer from Ideary's GitHub releases and run it; the app closes and reopens updated.
#[tauri::command]
async fn install_update(app: AppHandle, url: String) -> Result<(), String> {
    if !url.starts_with("https://github.com/ideary-app/ideary-app.github.io/releases/download/") {
        return Err("Only Ideary's own releases can be installed.".into());
    }
    let bytes = reqwest::get(&url).await.map_err(|e| e.to_string())?
        .error_for_status().map_err(|e| e.to_string())?
        .bytes().await.map_err(|e| e.to_string())?;
    let path = std::env::temp_dir().join("ideary-update-setup.exe");
    std::fs::write(&path, &bytes).map_err(|e| e.to_string())?;
    // /P = passive (small progress window), /R = reopen Ideary when done
    std::process::Command::new(&path).args(["/P", "/R"]).spawn().map_err(|e| e.to_string())?;
    app.exit(0);
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![app_version, popout, set_on_top, close_window, install_update])
        .run(tauri::generate_context!())
        .expect("error while running Ideary");
}
