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

// ---------- Google sign-in for the desktop app ----------
// Google's web popup can't open inside the app, so we sign in like an installed app:
// open the normal browser, catch the answer on a local port, keep a refresh token
// so Drive never "disconnects" again.
// Filled in at build time from the repo's Actions secrets (kept out of the code).
const G_CLIENT_ID: &str = match option_env!("IDEARY_G_CLIENT_ID") { Some(v) => v, None => "" };
// Desktop-app client "secret": Google documents it is not treated as secret for installed apps.
const G_CLIENT_SECRET: &str = match option_env!("IDEARY_G_CLIENT_SECRET") { Some(v) => v, None => "" };
const G_SCOPE: &str = "https://www.googleapis.com/auth/drive.file https://www.googleapis.com/auth/userinfo.email openid";

fn g_store(app: &AppHandle) -> Result<std::path::PathBuf, String> {
    let dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir.join("google.json"))
}

fn b64(bytes: &[u8]) -> String {
    use base64::Engine;
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes)
}

#[derive(serde::Serialize)]
struct GToken { access_token: String, expires_in: u64 }

async fn g_token_request(params: &[(&str, &str)]) -> Result<serde_json::Value, String> {
    let res = reqwest::Client::new().post("https://oauth2.googleapis.com/token").form(params).send().await.map_err(|e| e.to_string())?;
    let status = res.status();
    let v: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
    if !status.is_success() { return Err(v.get("error").and_then(|e| e.as_str()).unwrap_or("token error").to_string()); }
    Ok(v)
}

fn g_pack(v: &serde_json::Value) -> Result<GToken, String> {
    Ok(GToken {
        access_token: v.get("access_token").and_then(|x| x.as_str()).ok_or("no access token")?.to_string(),
        expires_in: v.get("expires_in").and_then(|x| x.as_u64()).unwrap_or(3600),
    })
}

/// Fresh access token from the saved refresh token, without showing anything. Err("no_session") if never signed in.
#[tauri::command]
async fn google_token(app: AppHandle) -> Result<GToken, String> {
    let path = g_store(&app)?;
    let raw = std::fs::read_to_string(&path).map_err(|_| "no_session".to_string())?;
    let saved: serde_json::Value = serde_json::from_str(&raw).map_err(|_| "no_session".to_string())?;
    let rt = saved.get("refresh_token").and_then(|x| x.as_str()).ok_or("no_session")?.to_string();
    match g_token_request(&[("client_id", G_CLIENT_ID), ("client_secret", G_CLIENT_SECRET), ("refresh_token", &rt), ("grant_type", "refresh_token")]).await {
        Ok(v) => g_pack(&v),
        Err(e) => { if e == "invalid_grant" { let _ = std::fs::remove_file(&path); return Err("no_session".into()); } Err(e) }
    }
}

/// Full sign-in: opens the browser, waits for Google to send the user back.
#[tauri::command]
async fn google_login(app: AppHandle, login_hint: Option<String>) -> Result<GToken, String> {
    if G_CLIENT_ID.is_empty() { return Err("This build has no Google sign-in set up.".into()); }
    use sha2::Digest;
    use std::io::{Read, Write};
    let mut rnd = [0u8; 48];
    getrandom::getrandom(&mut rnd).map_err(|e| e.to_string())?;
    let verifier = b64(&rnd);
    let challenge = b64(&sha2::Sha256::digest(verifier.as_bytes()));
    let mut st = [0u8; 16];
    getrandom::getrandom(&mut st).map_err(|e| e.to_string())?;
    let state = b64(&st);
    let listener = std::net::TcpListener::bind("127.0.0.1:0").map_err(|e| e.to_string())?;
    let port = listener.local_addr().map_err(|e| e.to_string())?.port();
    let redirect = format!("http://127.0.0.1:{port}");
    let enc = |s: &str| s.bytes().map(|b| if b.is_ascii_alphanumeric() || b"-_.~".contains(&b) { (b as char).to_string() } else { format!("%{:02X}", b) }).collect::<String>();
    let mut url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth?client_id={}&redirect_uri={}&response_type=code&scope={}&code_challenge={}&code_challenge_method=S256&state={}&access_type=offline&prompt=consent%20select_account",
        enc(G_CLIENT_ID), enc(&redirect), enc(G_SCOPE), challenge, state);
    if let Some(h) = login_hint.filter(|h| !h.is_empty()) { url.push_str(&format!("&login_hint={}", enc(&h))); }
    open::that(&url).map_err(|e| format!("Couldn't open the browser: {e}"))?;

    let expected = state.clone();
    let code = tauri::async_runtime::spawn_blocking(move || -> Result<String, String> {
        listener.set_nonblocking(false).ok();
        let started = std::time::Instant::now();
        loop {
            if started.elapsed().as_secs() > 300 { return Err("timeout".into()); }
            let (mut sock, _) = listener.accept().map_err(|e| e.to_string())?;
            let mut buf = [0u8; 4096];
            let n = sock.read(&mut buf).unwrap_or(0);
            let req = String::from_utf8_lossy(&buf[..n]).to_string();
            let line = req.lines().next().unwrap_or("");
            let target = line.split_whitespace().nth(1).unwrap_or("");
            let query = target.splitn(2, '?').nth(1).unwrap_or("");
            let mut code = None; let mut st = None; let mut err = None;
            for kv in query.split('&') {
                let mut it = kv.splitn(2, '=');
                let (k, v) = (it.next().unwrap_or(""), it.next().unwrap_or(""));
                let v = v.replace("%2F", "/").replace("%2f", "/");
                match k { "code" => code = Some(v), "state" => st = Some(v), "error" => err = Some(v), _ => {} }
            }
            if code.is_none() && err.is_none() { let _ = sock.write_all(b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n"); continue; }
            let ok = err.is_none() && st.as_deref() == Some(expected.as_str());
            let body = if ok { "<html><body style='font-family:sans-serif;text-align:center;padding-top:80px;background:#fbf6e8'><h2>You're signed in to Ideary</h2><p>You can close this tab and go back to the app.</p></body></html>" }
                       else { "<html><body style='font-family:sans-serif;text-align:center;padding-top:80px'><h2>Sign-in was cancelled</h2><p>Go back to Ideary and try again.</p></body></html>" };
            let _ = sock.write_all(format!("HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}", body.len(), body).as_bytes());
            if !ok { return Err(err.unwrap_or_else(|| "state mismatch".into())); }
            return Ok(code.unwrap());
        }
    }).await.map_err(|e| e.to_string())??;

    let v = g_token_request(&[("client_id", G_CLIENT_ID), ("client_secret", G_CLIENT_SECRET), ("code", &code), ("code_verifier", &verifier), ("redirect_uri", &redirect), ("grant_type", "authorization_code")]).await?;
    if let Some(rt) = v.get("refresh_token").and_then(|x| x.as_str()) {
        std::fs::write(g_store(&app)?, serde_json::json!({ "refresh_token": rt }).to_string()).map_err(|e| e.to_string())?;
    }
    if let Some(w) = app.get_webview_window("main") { let _ = w.unminimize(); let _ = w.set_focus(); }
    g_pack(&v)
}

#[tauri::command]
fn google_logout(app: AppHandle) -> Result<(), String> {
    let p = g_store(&app)?;
    if p.exists() { std::fs::remove_file(p).map_err(|e| e.to_string())?; }
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![app_version, popout, set_on_top, close_window, install_update, google_token, google_login, google_logout])
        .run(tauri::generate_context!())
        .expect("error while running Ideary");
}
