use crate::model_loader::ModelError;
use haetae_core::config::{
    DEFAULT_OLLAMA_HOST, HTTP_OK, OLLAMA_CMD_BREW_INSTALL, OLLAMA_CMD_CURL_INSTALL,
    OLLAMA_CMD_WINGET_INSTALL, OLLAMA_CONNECT_TIMEOUT_SECS, OLLAMA_DOWNLOAD_PAGE_URL,
    OLLAMA_POLL_INTERVAL_MS, OLLAMA_POLL_MAX_ATTEMPTS, OLLAMA_PROBE_BODY_MARKER,
    OLLAMA_PROBE_ENDPOINT,
};

#[cfg(windows)]
use haetae_core::config::{WIN_CREATE_NO_WINDOW_FLAG, WIN_DETACHED_PROCESS_FLAG};
use std::env;
use std::path::{Path, PathBuf};
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OllamaStatus {
    Running { host: String },
    Stopped { binary_path: PathBuf },
    NotInstalled,
}

pub struct OllamaLocator;

impl OllamaLocator {
    pub fn find_binary() -> Option<PathBuf> {
        let exe_name = if cfg!(windows) {
            "ollama.exe"
        } else {
            "ollama"
        };

        if let Some(paths) = env::var_os("PATH") {
            for dir in env::split_paths(&paths) {
                let candidate = dir.join(exe_name);
                if candidate.is_file() {
                    return Some(candidate);
                }
            }
        }

        #[cfg(windows)]
        {
            if let Some(local_app_data) = env::var_os("LOCALAPPDATA") {
                let p = Path::new(&local_app_data)
                    .join("Programs")
                    .join("Ollama")
                    .join("ollama.exe");
                if p.is_file() {
                    return Some(p);
                }
            }
            if let Some(pf) = env::var_os("ProgramFiles") {
                let p = Path::new(&pf).join("Ollama").join("ollama.exe");
                if p.is_file() {
                    return Some(p);
                }
            }
        }

        #[cfg(target_os = "macos")]
        {
            let mac_candidates = [
                "/Applications/Ollama.app/Contents/Resources/ollama",
                "/opt/homebrew/bin/ollama",
                "/usr/local/bin/ollama",
                "/usr/bin/ollama",
            ];
            for c in mac_candidates {
                let p = PathBuf::from(c);
                if p.is_file() {
                    return Some(p);
                }
            }
        }

        #[cfg(all(unix, not(target_os = "macos")))]
        {
            let linux_candidates = ["/usr/local/bin/ollama", "/usr/bin/ollama", "/bin/ollama"];
            for c in linux_candidates {
                let p = PathBuf::from(c);
                if p.is_file() {
                    return Some(p);
                }
            }
            if let Some(home) = env::var_os("HOME") {
                let p = Path::new(&home).join(".local").join("bin").join("ollama");
                if p.is_file() {
                    return Some(p);
                }
            }
        }

        None
    }

    pub fn get_install_command() -> &'static str {
        if cfg!(windows) {
            OLLAMA_CMD_WINGET_INSTALL
        } else if cfg!(target_os = "macos") {
            OLLAMA_CMD_BREW_INSTALL
        } else {
            OLLAMA_CMD_CURL_INSTALL
        }
    }

    pub fn get_download_url() -> &'static str {
        OLLAMA_DOWNLOAD_PAGE_URL
    }
}

pub struct OllamaDaemonManager {
    host: String,
    client: reqwest::Client,
}

impl Default for OllamaDaemonManager {
    fn default() -> Self {
        Self::new(DEFAULT_OLLAMA_HOST)
    }
}

impl OllamaDaemonManager {
    pub fn new(host: impl Into<String>) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(OLLAMA_CONNECT_TIMEOUT_SECS))
            .connect_timeout(Duration::from_secs(OLLAMA_CONNECT_TIMEOUT_SECS))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());

        Self {
            host: host.into(),
            client,
        }
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub async fn is_alive(&self) -> bool {
        let probe_url = format!("{}{}", self.host, OLLAMA_PROBE_ENDPOINT);
        match self.client.get(&probe_url).send().await {
            Ok(resp) if resp.status().as_u16() == HTTP_OK => {
                if let Ok(text) = resp.text().await {
                    text.contains(OLLAMA_PROBE_BODY_MARKER)
                } else {
                    true
                }
            }
            _ => false,
        }
    }

    pub async fn check_status(&self) -> OllamaStatus {
        if self.is_alive().await {
            return OllamaStatus::Running {
                host: self.host.clone(),
            };
        }

        if let Some(binary_path) = OllamaLocator::find_binary() {
            OllamaStatus::Stopped { binary_path }
        } else {
            OllamaStatus::NotInstalled
        }
    }

    pub async fn spawn_daemon(&self) -> Result<(), ModelError> {
        let binary_path = OllamaLocator::find_binary().ok_or_else(|| {
            ModelError::Inference(format!(
                "Ollama is not installed. Install via `{}` or download from {}",
                OllamaLocator::get_install_command(),
                OLLAMA_DOWNLOAD_PAGE_URL
            ))
        })?;

        let mut cmd = std::process::Command::new(&binary_path);
        cmd.arg("serve")
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null());

        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            cmd.creation_flags(WIN_CREATE_NO_WINDOW_FLAG | WIN_DETACHED_PROCESS_FLAG);
        }

        cmd.spawn().map_err(|e| {
            ModelError::Inference(format!(
                "Failed to spawn Ollama daemon at {:?}: {}",
                binary_path, e
            ))
        })?;

        for _ in 0..OLLAMA_POLL_MAX_ATTEMPTS {
            tokio::time::sleep(Duration::from_millis(OLLAMA_POLL_INTERVAL_MS)).await;
            if self.is_alive().await {
                return Ok(());
            }
        }

        Err(ModelError::Inference(format!(
            "Ollama daemon spawned at {:?} but did not become ready at {} within expected timeout.",
            binary_path, self.host
        )))
    }

    pub async fn ensure_running(&self) -> Result<(), ModelError> {
        if self.is_alive().await {
            return Ok(());
        }
        self.spawn_daemon().await
    }
}
