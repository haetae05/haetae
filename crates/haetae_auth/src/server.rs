use crate::client::{AuthError, LicenseRequest, LicenseResponse, LicenseTier};
use haetae_core::config::{
    BUFFER_CAPACITY_BYTES, DEFAULT_LICENSE_EXPIRY_TIMESTAMP, HTTP_BAD_REQUEST, HTTP_INTERNAL_ERROR,
    HTTP_NOT_FOUND, HTTP_OK, LICENSE_VALIDITY_DAYS, SECS_PER_DAY, SERVER_BUFFER_CHUNK_SIZE,
    SERVER_PORT, SIZE_HAETAE_EXE_BYTES, SIZE_HAETAE_LINUX_ARM64_BYTES, SIZE_HAETAE_VSIX_BYTES,
};
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

pub use crate::client::{ROUTE_AUTH_VERIFY, ROUTE_LICENSE_VERIFY};

const HTTP_REQUEST_PARTS_SPEC: &[&str] = &["METHOD", "PATH", "VERSION"];
const MIN_HTTP_REQUEST_LINE_PARTS: usize = HTTP_REQUEST_PARTS_SPEC.len();
const CRLF_CRLF_DELIM: &str = "\r\n\r\n";
const LF_LF_DELIM: &str = "\n\n";
const CRLF_CRLF_LEN: usize = CRLF_CRLF_DELIM.len();
const LF_LF_LEN: usize = LF_LF_DELIM.len();

pub const ROUTE_HEALTH: &str = "/health";
pub const ROUTE_API_HEALTH: &str = "/api/v1/health";
pub const ROUTE_ROOT: &str = "/";

pub const HTTP_STATUS_OK: u16 = HTTP_OK;
pub const HTTP_STATUS_BAD_REQUEST: u16 = HTTP_BAD_REQUEST;
pub const HTTP_STATUS_NOT_FOUND: u16 = HTTP_NOT_FOUND;
pub const HTTP_STATUS_INTERNAL_ERROR: u16 = HTTP_INTERNAL_ERROR;

pub const STATUS_LINE_OK: &str = "HTTP/1.1 200 OK";
pub const STATUS_LINE_BAD_REQUEST: &str = "HTTP/1.1 400 Bad Request";
pub const STATUS_LINE_NOT_FOUND: &str = "HTTP/1.1 404 Not Found";
pub const STATUS_LINE_INTERNAL_ERROR: &str = "HTTP/1.1 500 Internal Server Error";

pub const HEADER_CORS_ORIGIN: &str = "Access-Control-Allow-Origin: *";
pub const HEADER_CORS_METHODS: &str = "Access-Control-Allow-Methods: GET, POST, HEAD, OPTIONS";
pub const HEADER_CORS_HEADERS: &str = "Access-Control-Allow-Headers: Content-Type, Authorization";
pub const HEADER_CONNECTION_CLOSE: &str = "Connection: close";

pub const JSON_ERROR_BAD_REQUEST: &str = "{\"error\": \"Bad Request\"}";
pub const JSON_ERROR_INVALID_BODY: &str = "{\"error\": \"Invalid JSON body\"}";
pub const JSON_ERROR_NOT_FOUND: &str = "{\"error\": \"Not Found\"}";
pub const JSON_HEALTH_PAYLOAD: &str =
    "{\"status\": \"healthy\", \"service\": \"haetae-license-server\"}";
pub const SERVER_PORT_DEFAULT: u16 = SERVER_PORT;
pub const BUFFER_CHUNK_SIZE: usize = SERVER_BUFFER_CHUNK_SIZE;

pub const SIZE_HAETAE_EXE: u64 = SIZE_HAETAE_EXE_BYTES;
pub const SIZE_HAETAE_LINUX_ARM64: u64 = SIZE_HAETAE_LINUX_ARM64_BYTES;
pub const SIZE_HAETAE_VSIX: u64 = SIZE_HAETAE_VSIX_BYTES;

pub const SHA256_HAETAE_EXE: &str =
    "4F95C0294F0ABA3B4B49EF54102A95F29E193C3B4060A598E4BBD406E11A02E5";
pub const SHA256_HAETAE_LINUX_ARM64: &str =
    "E9A77EB37EE48859A5D8FAE0216FCEC81A3FDD286F07BFD17CAFE57B5E7ADFD4";
pub const SHA256_HAETAE_VSIX: &str =
    "9D9D2FA5C13B97F7BCB5F6338CD1AEC71A88ABB753F2740813262C7F36E4BB60";

pub const ROUTE_HAETAE_EXE: &str = "/downloads/haetae.exe";
pub const ROUTE_HAETAE_EXE_ROOT: &str = "/haetae.exe";
pub const ROUTE_HAETAE_ARM64: &str = "/downloads/haetae-linux-arm64";
pub const ROUTE_HAETAE_ARM64_ROOT: &str = "/haetae-linux-arm64";
pub const ROUTE_HAETAE_VSIX: &str = "/downloads/haetae-0.1.0.vsix";
pub const ROUTE_HAETAE_VSIX_ROOT: &str = "/haetae-0.1.0.vsix";

pub const FILE_NAME_HAETAE_EXE: &str = "haetae.exe";
pub const FILE_NAME_HAETAE_ARM64: &str = "haetae-linux-arm64";
pub const FILE_NAME_HAETAE_VSIX: &str = "haetae-0.1.0.vsix";

pub const MIME_OCTET_STREAM: &str = "application/octet-stream";
pub const MIME_VSIX: &str = "application/vnd.code.vsix+zip";
pub const MIME_HTML: &str = "text/html; charset=utf-8";
pub const MIME_JS: &str = "text/javascript; charset=utf-8";
pub const MIME_CSS: &str = "text/css; charset=utf-8";
pub const MIME_WEBP: &str = "image/webp";
pub const MIME_PNG: &str = "image/png";
pub const MIME_JPEG: &str = "image/jpeg";
pub const MIME_SVG: &str = "image/svg+xml";
pub const MIME_JSON: &str = "application/json";
pub const MIME_ICO: &str = "image/x-icon";
pub const MIME_WOFF: &str = "font/woff";
pub const MIME_WOFF2: &str = "font/woff2";
pub const MIME_TTF: &str = "font/ttf";

pub struct LicenseServer {
    pub port: u16,
    pub web_dir: Option<PathBuf>,
}

impl Default for LicenseServer {
    fn default() -> Self {
        Self {
            port: SERVER_PORT_DEFAULT,
            web_dir: resolve_web_dir(None),
        }
    }
}

impl LicenseServer {
    pub fn new(port: u16) -> Self {
        Self {
            port,
            web_dir: resolve_web_dir(None),
        }
    }

    pub fn with_web_dir(port: u16, web_dir: Option<PathBuf>) -> Self {
        Self {
            port,
            web_dir: resolve_web_dir(web_dir.as_deref()),
        }
    }

    pub async fn bind_listener(&self) -> Result<TcpListener, AuthError> {
        let addr = SocketAddr::from(([0, 0, 0, 0], self.port));
        TcpListener::bind(addr)
            .await
            .map_err(|e| AuthError::Network(e.to_string()))
    }

    pub async fn serve(&self, listener: TcpListener) -> Result<(), AuthError> {
        let web_dir = self.web_dir.clone();
        loop {
            let (stream, _) = match listener.accept().await {
                Ok(conn) => conn,
                Err(_) => continue,
            };

            let dir_clone = web_dir.clone();
            tokio::spawn(async move {
                let _ = handle_connection(stream, dir_clone).await;
            });
        }
    }

    pub async fn run(&self) -> Result<(), AuthError> {
        let listener = self.bind_listener().await?;
        if let Ok(local_addr) = listener.local_addr() {
            println!("Haetae License Server listening on {}", local_addr);
        }
        self.serve(listener).await
    }
}

pub fn resolve_web_dir(custom_dir: Option<&Path>) -> Option<PathBuf> {
    if let Some(d) = custom_dir {
        if d.is_dir() {
            return Some(d.to_path_buf());
        }
    }
    if let Ok(env_dir) = std::env::var("HAETAE_WEB_DIR") {
        let p = PathBuf::from(env_dir);
        if p.is_dir() {
            return Some(p);
        }
    }
    let candidate_paths = [
        PathBuf::from("dist"),
        PathBuf::from("../dist"),
        PathBuf::from("../../dist"),
        PathBuf::from("../../../dist"),
        PathBuf::from("04_공식웹사이트/dist"),
        PathBuf::from("../04_공식웹사이트/dist"),
        PathBuf::from("../../04_공식웹사이트/dist"),
    ];
    if let Some(found) = candidate_paths.into_iter().find(|p| p.is_dir()) {
        return Some(found);
    }
    if let Ok(home) = std::env::var("HOME") {
        let home_p = PathBuf::from(home);
        let user_candidates = [
            home_p.join("haetae05/dist"),
            home_p.join("haetae_website/dist"),
            home_p.join("dist"),
        ];
        if let Some(found) = user_candidates.into_iter().find(|p| p.is_dir()) {
            return Some(found);
        }
    }
    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let m_path = PathBuf::from(manifest_dir);
        let cargo_candidates = [
            m_path.join("../../../04_공식웹사이트/dist"),
            m_path.join("../../04_공식웹사이트/dist"),
            m_path.join("../04_공식웹사이트/dist"),
            m_path.join("dist"),
            m_path.join("../dist"),
            m_path.join("../../dist"),
            m_path.join("../../../dist"),
        ];
        if let Some(found) = cargo_candidates.into_iter().find(|p| p.is_dir()) {
            return Some(found);
        }
    }
    None
}

pub fn resolve_binary_artifact(filename: &str, web_dir: Option<&Path>) -> Option<PathBuf> {
    if let Some(dir) = web_dir {
        let p_downloads = dir.join("downloads").join(filename);
        if p_downloads.is_file() {
            return Some(p_downloads);
        }
        let p_direct = dir.join(filename);
        if p_direct.is_file() {
            return Some(p_direct);
        }
        if let Some(parent) = dir.parent() {
            if let Some(root) = parent.parent() {
                let p_root_bin = root.join("01_바이너리_실행파일").join(filename);
                if p_root_bin.is_file() {
                    return Some(p_root_bin);
                }
                let p_root_vsix = root.join("02_VSCode_확장프로그램").join(filename);
                if p_root_vsix.is_file() {
                    return Some(p_root_vsix);
                }
            }
        }
    }

    if let Ok(dir_env) = std::env::var("HAETAE_DOWNLOADS_DIR") {
        let p = PathBuf::from(dir_env).join(filename);
        if p.is_file() {
            return Some(p);
        }
    }

    let candidates = [
        PathBuf::from("01_바이너리_실행파일").join(filename),
        PathBuf::from("02_VSCode_확장프로그램").join(filename),
        PathBuf::from("../01_바이너리_실행파일").join(filename),
        PathBuf::from("../02_VSCode_확장프로그램").join(filename),
        PathBuf::from("../../01_바이너리_실행파일").join(filename),
        PathBuf::from("../../02_VSCode_확장프로그램").join(filename),
        PathBuf::from("../../../01_바이너리_실행파일").join(filename),
        PathBuf::from("../../../02_VSCode_확장프로그램").join(filename),
        PathBuf::from("dist/downloads").join(filename),
        PathBuf::from("../dist/downloads").join(filename),
        PathBuf::from("../../../dist/downloads").join(filename),
    ];

    for c in candidates.into_iter() {
        if c.is_file() {
            return Some(c);
        }
    }

    if let Ok(home) = std::env::var("HOME") {
        let home_p = PathBuf::from(home);
        let user_candidates = [
            home_p.join("haetae05/dist/downloads").join(filename),
            home_p.join("haetae05/01_바이너리_실행파일").join(filename),
            home_p
                .join("haetae05/02_VSCode_확장프로그램")
                .join(filename),
            home_p.join("dist/downloads").join(filename),
        ];
        for c in user_candidates.into_iter() {
            if c.is_file() {
                return Some(c);
            }
        }
    }

    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let m_path = PathBuf::from(manifest_dir);
        let cargo_candidates = [
            m_path.join("../../../01_바이너리_실행파일").join(filename),
            m_path
                .join("../../../02_VSCode_확장프로그램")
                .join(filename),
            m_path.join("../../01_바이너리_실행파일").join(filename),
            m_path.join("../../02_VSCode_확장프로그램").join(filename),
            m_path.join("../01_바이너리_실행파일").join(filename),
            m_path.join("../02_VSCode_확장프로그램").join(filename),
            m_path.join("../../../dist/downloads").join(filename),
            m_path.join("../../dist/downloads").join(filename),
        ];
        for c in cargo_candidates.into_iter() {
            if c.is_file() {
                return Some(c);
            }
        }
    }

    None
}

pub fn get_mime_type(path_str: &str) -> &'static str {
    if path_str.ends_with(".html") || path_str.ends_with(".htm") {
        MIME_HTML
    } else if path_str.ends_with(".js") || path_str.ends_with(".mjs") {
        MIME_JS
    } else if path_str.ends_with(".css") {
        MIME_CSS
    } else if path_str.ends_with(".webp") {
        MIME_WEBP
    } else if path_str.ends_with(".png") {
        MIME_PNG
    } else if path_str.ends_with(".jpg") || path_str.ends_with(".jpeg") {
        MIME_JPEG
    } else if path_str.ends_with(".svg") {
        MIME_SVG
    } else if path_str.ends_with(".json") {
        MIME_JSON
    } else if path_str.ends_with(".ico") {
        MIME_ICO
    } else if path_str.ends_with(".woff") {
        MIME_WOFF
    } else if path_str.ends_with(".woff2") {
        MIME_WOFF2
    } else if path_str.ends_with(".ttf") {
        MIME_TTF
    } else if path_str.ends_with(".vsix") {
        MIME_VSIX
    } else {
        MIME_OCTET_STREAM
    }
}

pub fn match_binary_route(path: &str) -> Option<(&'static str, &'static str)> {
    if path == ROUTE_HAETAE_EXE || path == ROUTE_HAETAE_EXE_ROOT {
        Some((FILE_NAME_HAETAE_EXE, MIME_OCTET_STREAM))
    } else if path == ROUTE_HAETAE_ARM64 || path == ROUTE_HAETAE_ARM64_ROOT {
        Some((FILE_NAME_HAETAE_ARM64, MIME_OCTET_STREAM))
    } else if path == ROUTE_HAETAE_VSIX || path == ROUTE_HAETAE_VSIX_ROOT {
        Some((FILE_NAME_HAETAE_VSIX, MIME_VSIX))
    } else {
        None
    }
}

async fn serve_binary_file(
    stream: &mut TcpStream,
    path: &Path,
    filename: &str,
    content_type: &str,
    method: &str,
) -> Result<(), AuthError> {
    let mut file = tokio::fs::File::open(path)
        .await
        .map_err(|e| AuthError::Network(e.to_string()))?;
    let metadata = file
        .metadata()
        .await
        .map_err(|e| AuthError::Network(e.to_string()))?;
    let file_len = metadata.len();

    let header = format!(
        "{}\r\n{}\r\n{}\r\n{}\r\nContent-Type: {}\r\nContent-Disposition: attachment; filename=\"{}\"\r\nContent-Length: {}\r\n{}\r\n\r\n",
        STATUS_LINE_OK,
        HEADER_CORS_ORIGIN,
        HEADER_CORS_METHODS,
        HEADER_CORS_HEADERS,
        content_type,
        filename,
        file_len,
        HEADER_CONNECTION_CLOSE
    );

    stream
        .write_all(header.as_bytes())
        .await
        .map_err(|e| AuthError::Network(e.to_string()))?;

    if method == "GET" {
        let mut chunk_buf = vec![0u8; BUFFER_CHUNK_SIZE];
        loop {
            let read_bytes = file
                .read(&mut chunk_buf)
                .await
                .map_err(|e| AuthError::Network(e.to_string()))?;
            if read_bytes == 0 {
                break;
            }
            stream
                .write_all(&chunk_buf[0..read_bytes])
                .await
                .map_err(|e| AuthError::Network(e.to_string()))?;
        }
    }

    stream
        .flush()
        .await
        .map_err(|e| AuthError::Network(e.to_string()))?;
    let _ = stream.shutdown().await;
    Ok(())
}

async fn serve_static_file(
    stream: &mut TcpStream,
    path: &Path,
    mime_type: &str,
    method: &str,
) -> Result<(), AuthError> {
    let mut file = tokio::fs::File::open(path)
        .await
        .map_err(|e| AuthError::Network(e.to_string()))?;
    let metadata = file
        .metadata()
        .await
        .map_err(|e| AuthError::Network(e.to_string()))?;
    let file_len = metadata.len();

    let header = format!(
        "{}\r\n{}\r\n{}\r\n{}\r\nContent-Type: {}\r\nContent-Length: {}\r\n{}\r\n\r\n",
        STATUS_LINE_OK,
        HEADER_CORS_ORIGIN,
        HEADER_CORS_METHODS,
        HEADER_CORS_HEADERS,
        mime_type,
        file_len,
        HEADER_CONNECTION_CLOSE
    );

    stream
        .write_all(header.as_bytes())
        .await
        .map_err(|e| AuthError::Network(e.to_string()))?;

    if method == "GET" {
        let mut chunk_buf = vec![0u8; BUFFER_CHUNK_SIZE];
        loop {
            let read_bytes = file
                .read(&mut chunk_buf)
                .await
                .map_err(|e| AuthError::Network(e.to_string()))?;
            if read_bytes == 0 {
                break;
            }
            stream
                .write_all(&chunk_buf[0..read_bytes])
                .await
                .map_err(|e| AuthError::Network(e.to_string()))?;
        }
    }

    stream
        .flush()
        .await
        .map_err(|e| AuthError::Network(e.to_string()))?;
    let _ = stream.shutdown().await;
    Ok(())
}

pub async fn verify_lemonsqueezy_license(license_key: &str, instance_name: &str) -> bool {
    let client = match reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(
            haetae_core::config::LEMONSQUEEZY_TIMEOUT_SECS,
        ))
        .build()
    {
        Ok(c) => c,
        Err(_) => return false,
    };

    let params = [
        ("license_key", license_key),
        ("instance_name", instance_name),
    ];

    if let Ok(resp) = client
        .post(haetae_core::config::LEMONSQUEEZY_ACTIVATE_URL)
        .header("Accept", "application/json")
        .form(&params)
        .send()
        .await
    {
        if let Ok(json) = resp.json::<serde_json::Value>().await {
            if json
                .get("activated")
                .and_then(|v| v.as_bool())
                .unwrap_or(false)
            {
                return true;
            }
        }
    }

    let val_params = [("license_key", license_key)];
    if let Ok(resp) = client
        .post(haetae_core::config::LEMONSQUEEZY_VALIDATE_URL)
        .header("Accept", "application/json")
        .form(&val_params)
        .send()
        .await
    {
        if let Ok(json) = resp.json::<serde_json::Value>().await {
            if json.get("valid").and_then(|v| v.as_bool()).unwrap_or(false) {
                let status = json
                    .get("license_key")
                    .and_then(|k| k.get("status"))
                    .and_then(|s| s.as_str());
                if status == Some(haetae_core::config::LEMONSQUEEZY_STATUS_ACTIVE) {
                    return true;
                }
            }
        }
    }

    false
}

pub fn resolve_license_secret() -> String {
    let secret = crate::crypto::get_license_secret();
    let cleaned = secret.trim().trim_matches('"').trim_matches('\'');
    if !cleaned.is_empty() {
        return cleaned.to_string();
    }
    if let Ok(alt) = std::env::var("HAETAE_AUTH_SECRET") {
        let cleaned_alt = alt.trim().trim_matches('"').trim_matches('\'');
        if !cleaned_alt.is_empty() {
            return cleaned_alt.to_string();
        }
    }
    String::new()
}

pub async fn handle_verify(body_str: &str) -> String {
    let req_data: Result<LicenseRequest, _> = serde_json::from_str(body_str.trim());
    match req_data {
        Ok(lic) => {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(DEFAULT_LICENSE_EXPIRY_TIMESTAMP);
            let expiry = now + (LICENSE_VALIDITY_DAYS * SECS_PER_DAY);
            let secret = resolve_license_secret();

            let mut verified_tier = if secret.is_empty() {
                Err(AuthError::InvalidLicense)
            } else {
                crate::crypto::verify_license_key(&lic.license_key, &secret)
            };

            if verified_tier.is_err()
                && verify_lemonsqueezy_license(&lic.license_key, &lic.machine_id).await
            {
                verified_tier = Ok(LicenseTier::Pro);
            }

            let resp_data = match verified_tier {
                Ok(tier) => {
                    let token = if secret.is_empty() {
                        None
                    } else {
                        Some(crate::crypto::generate_license_token(
                            &lic.license_key,
                            &lic.machine_id,
                            tier,
                            expiry,
                            &secret,
                        ))
                    };
                    LicenseResponse {
                        valid: true,
                        tier,
                        expires_at: expiry,
                        token,
                    }
                }
                Err(_) => LicenseResponse {
                    valid: false,
                    tier: LicenseTier::Community,
                    expires_at: expiry,
                    token: None,
                },
            };
            let json = serde_json::to_string(&resp_data).unwrap_or_default();
            build_http_response(HTTP_STATUS_OK, &json)
        }
        Err(_) => build_http_response(HTTP_STATUS_BAD_REQUEST, JSON_ERROR_INVALID_BODY),
    }
}

async fn handle_connection(
    mut stream: TcpStream,
    web_dir: Option<PathBuf>,
) -> Result<(), AuthError> {
    let mut header_buf = Vec::new();
    let mut header_end_idx = None;
    let mut delim_len = 0;

    let mut temp_chunk = [0u8; BUFFER_CAPACITY_BYTES];
    loop {
        let n = stream
            .read(&mut temp_chunk)
            .await
            .map_err(|e| AuthError::Network(e.to_string()))?;
        if n == 0 {
            break;
        }
        header_buf.extend_from_slice(&temp_chunk[0..n]);

        if let Some(pos) = header_buf
            .windows(CRLF_CRLF_LEN)
            .position(|w| w == b"\r\n\r\n")
        {
            header_end_idx = Some(pos);
            delim_len = CRLF_CRLF_LEN;
            break;
        }
        if let Some(pos) = header_buf.windows(LF_LF_LEN).position(|w| w == b"\n\n") {
            header_end_idx = Some(pos);
            delim_len = LF_LF_LEN;
            break;
        }
    }

    if header_buf.is_empty() {
        return Ok(());
    }

    let (headers_str, initial_body) = if let Some(end_idx) = header_end_idx {
        let body_start = end_idx + delim_len;
        let h_str = String::from_utf8_lossy(&header_buf[0..end_idx]).to_string();
        let b_slice = header_buf[body_start..].to_vec();
        (h_str, b_slice)
    } else {
        (String::from_utf8_lossy(&header_buf).to_string(), Vec::new())
    };

    let mut content_length: usize = 0;
    for line in headers_str.lines() {
        let lower = line.to_lowercase();
        if let Some(val_str) = lower.strip_prefix("content-length:") {
            if let Ok(len) = val_str.trim().parse::<usize>() {
                content_length = len;
            }
        }
    }

    let mut body_buf = initial_body;
    while body_buf.len() < content_length {
        let remaining = content_length - body_buf.len();
        let to_read = remaining.min(BUFFER_CAPACITY_BYTES);
        let n = stream
            .read(&mut temp_chunk[0..to_read])
            .await
            .map_err(|e| AuthError::Network(e.to_string()))?;
        if n == 0 {
            break;
        }
        body_buf.extend_from_slice(&temp_chunk[0..n]);
    }

    let body_str = String::from_utf8_lossy(&body_buf);

    let mut lines = headers_str.lines();
    let first_line = lines.next().unwrap_or_default();
    let parts: Vec<&str> = first_line.split_whitespace().collect();

    if parts.len() < MIN_HTTP_REQUEST_LINE_PARTS {
        let resp = build_http_response(HTTP_STATUS_BAD_REQUEST, JSON_ERROR_BAD_REQUEST);
        let _ = stream.write_all(resp.as_bytes()).await;
        return Ok(());
    }

    let method = parts[0];
    let raw_path = parts[1];
    let path = raw_path.split('?').next().unwrap_or(raw_path);
    let normalized_path = path.strip_suffix('/').unwrap_or(path);

    if method == "OPTIONS" {
        let cors_resp = format!(
            "{}\r\n{}\r\n{}\r\n{}\r\nContent-Length: 0\r\n{}\r\n\r\n",
            STATUS_LINE_OK,
            HEADER_CORS_ORIGIN,
            HEADER_CORS_METHODS,
            HEADER_CORS_HEADERS,
            HEADER_CONNECTION_CLOSE
        );
        let _ = stream.write_all(cors_resp.as_bytes()).await;
        let _ = stream.flush().await;
        let _ = stream.shutdown().await;
        return Ok(());
    }

    if method == "POST"
        && (path == ROUTE_AUTH_VERIFY
            || path == ROUTE_LICENSE_VERIFY
            || normalized_path == ROUTE_AUTH_VERIFY
            || normalized_path == ROUTE_LICENSE_VERIFY)
    {
        let resp_str = handle_verify(&body_str).await;
        let _ = stream.write_all(resp_str.as_bytes()).await;
        let _ = stream.flush().await;
        let _ = stream.shutdown().await;
        return Ok(());
    }

    if (method == "GET" || method == "HEAD")
        && (path == ROUTE_HEALTH
            || path == ROUTE_API_HEALTH
            || normalized_path == ROUTE_HEALTH
            || normalized_path == ROUTE_API_HEALTH)
    {
        let resp = format!(
            "{}\r\n{}\r\n{}\r\n{}\r\nContent-Type: {}\r\nContent-Length: {}\r\n{}\r\n\r\n",
            STATUS_LINE_OK,
            HEADER_CORS_ORIGIN,
            HEADER_CORS_METHODS,
            HEADER_CORS_HEADERS,
            MIME_JSON,
            JSON_HEALTH_PAYLOAD.len(),
            HEADER_CONNECTION_CLOSE
        );
        let _ = stream.write_all(resp.as_bytes()).await;
        if method == "GET" {
            let _ = stream.write_all(JSON_HEALTH_PAYLOAD.as_bytes()).await;
        }
        let _ = stream.flush().await;
        let _ = stream.shutdown().await;
        return Ok(());
    }

    if method == "GET" || method == "HEAD" {
        if let Some((filename, content_type)) = match_binary_route(path) {
            if let Some(fp) = resolve_binary_artifact(filename, web_dir.as_deref()) {
                serve_binary_file(&mut stream, &fp, filename, content_type, method).await?;
                return Ok(());
            }
        }
        if let Some(relative_dl) = path.strip_prefix("/downloads/") {
            if !relative_dl.is_empty() && !relative_dl.contains("..") {
                let content_type = get_mime_type(relative_dl);
                if let Some(fp) = resolve_binary_artifact(relative_dl, web_dir.as_deref()) {
                    serve_binary_file(&mut stream, &fp, relative_dl, content_type, method).await?;
                    return Ok(());
                }
            }
        }

        if let Some(ref dir) = web_dir {
            let clean_path = path.trim_start_matches('/');
            if clean_path.contains("..") {
                let resp = build_http_response(HTTP_STATUS_NOT_FOUND, JSON_ERROR_NOT_FOUND);
                let _ = stream.write_all(resp.as_bytes()).await;
                let _ = stream.flush().await;
                let _ = stream.shutdown().await;
                return Ok(());
            }

            let target_file = if clean_path.is_empty() {
                dir.join("index.html")
            } else {
                dir.join(clean_path)
            };

            let file_to_serve = if let (Ok(canon_target), Ok(canon_dir)) =
                (target_file.canonicalize(), dir.canonicalize())
            {
                if canon_target.starts_with(&canon_dir) && canon_target.is_file() {
                    Some(canon_target)
                } else {
                    None
                }
            } else if !clean_path.contains('.') {
                let index = dir.join("index.html");
                if let (Ok(canon_index), Ok(canon_dir)) = (index.canonicalize(), dir.canonicalize())
                {
                    if canon_index.starts_with(&canon_dir) && canon_index.is_file() {
                        Some(canon_index)
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            };

            if let Some(fp) = file_to_serve {
                let mime = get_mime_type(&fp.to_string_lossy());
                serve_static_file(&mut stream, &fp, mime, method).await?;
                return Ok(());
            }
        }

        if path == ROUTE_ROOT {
            let resp = format!(
                "{}\r\n{}\r\n{}\r\n{}\r\nContent-Type: {}\r\nContent-Length: {}\r\n{}\r\n\r\n",
                STATUS_LINE_OK,
                HEADER_CORS_ORIGIN,
                HEADER_CORS_METHODS,
                HEADER_CORS_HEADERS,
                MIME_JSON,
                JSON_HEALTH_PAYLOAD.len(),
                HEADER_CONNECTION_CLOSE
            );
            let _ = stream.write_all(resp.as_bytes()).await;
            if method == "GET" {
                let _ = stream.write_all(JSON_HEALTH_PAYLOAD.as_bytes()).await;
            }
            let _ = stream.flush().await;
            let _ = stream.shutdown().await;
            return Ok(());
        }
    }

    let resp = build_http_response(HTTP_STATUS_NOT_FOUND, JSON_ERROR_NOT_FOUND);
    let _ = stream.write_all(resp.as_bytes()).await;
    let _ = stream.flush().await;
    let _ = stream.shutdown().await;
    Ok(())
}

pub fn build_http_response(status: u16, json_body: &str) -> String {
    let status_line = match status {
        HTTP_STATUS_OK => STATUS_LINE_OK,
        HTTP_STATUS_BAD_REQUEST => STATUS_LINE_BAD_REQUEST,
        HTTP_STATUS_NOT_FOUND => STATUS_LINE_NOT_FOUND,
        _ => STATUS_LINE_INTERNAL_ERROR,
    };

    format!(
        "{}\r\n{}\r\n{}\r\n{}\r\nContent-Type: {}\r\nContent-Length: {}\r\n{}\r\n\r\n{}",
        status_line,
        HEADER_CORS_ORIGIN,
        HEADER_CORS_METHODS,
        HEADER_CORS_HEADERS,
        MIME_JSON,
        json_body.len(),
        HEADER_CONNECTION_CLOSE,
        json_body
    )
}
