use haetae_core::config::{
    DEFAULT_ORACLE_SERVER_HOST, ENV_HAETAE_LICENSE_FILE, ENV_HAETAE_LICENSE_TOKEN,
    HTTP_CONNECT_TIMEOUT_SECS, HTTP_TIMEOUT_SECS, OFFLINE_GRACE_PERIOD_SECS, TOKEN_PARTS_COUNT,
};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU8, Ordering};
use std::time::Duration;

pub const ROUTE_AUTH_VERIFY: &str = "/api/v1/auth/verify";
pub const ROUTE_LICENSE_VERIFY: &str = "/api/v1/license/verify";

const TIER_CODE_COMMUNITY: u8 = 0;
const TIER_CODE_PRO: u8 = 1;

pub use crate::machine_id::get_hardware_machine_id;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum LicenseTier {
    Community,
    Pro,
}

impl LicenseTier {
    pub fn is_pro(&self) -> bool {
        matches!(self, LicenseTier::Pro)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LicenseRequest {
    pub license_key: String,
    pub machine_id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LicenseResponse {
    pub valid: bool,
    pub tier: LicenseTier,
    pub expires_at: u64,
    pub token: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct StoredLicense {
    pub license_key: String,
    pub token: String,
    pub tier: LicenseTier,
    pub last_verified_at: u64,
    pub expires_at: u64,
}

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Network request failed: {0}")]
    Network(String),
    #[error("Serialization failed: {0}")]
    Serialization(String),
    #[error("Storage error: {0}")]
    Storage(String),
    #[error("Invalid license key")]
    InvalidLicense,
    #[error("Server error: {0}")]
    ServerError(u16),
}

pub fn get_license_file_path() -> PathBuf {
    if let Ok(p) = std::env::var(ENV_HAETAE_LICENSE_FILE) {
        return PathBuf::from(p);
    }
    if let Ok(home) = std::env::var("HOME").or_else(|_| std::env::var("USERPROFILE")) {
        PathBuf::from(home).join(".haetae").join("license.jwt")
    } else {
        PathBuf::from(".haetae").join("license.jwt")
    }
}

pub fn load_stored_license() -> Option<StoredLicense> {
    let p = get_license_file_path();
    if !p.exists() {
        return None;
    }
    let data = std::fs::read_to_string(p).ok()?;
    serde_json::from_str(&data).ok()
}

pub fn save_stored_license(lic: &StoredLicense) -> Result<(), AuthError> {
    let p = get_license_file_path();
    if let Some(parent) = p.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let json = serde_json::to_string(lic).map_err(|e| AuthError::Serialization(e.to_string()))?;
    std::fs::write(p, json).map_err(|e| AuthError::Storage(e.to_string()))?;
    Ok(())
}

pub struct AuthClient {
    pub server_url: String,
    pub client: reqwest::Client,
    cached_tier_code: AtomicU8,
}

impl Default for AuthClient {
    fn default() -> Self {
        Self::new(DEFAULT_ORACLE_SERVER_HOST)
    }
}

impl AuthClient {
    pub fn new(server_url: impl Into<String>) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(HTTP_TIMEOUT_SECS))
            .connect_timeout(Duration::from_secs(HTTP_CONNECT_TIMEOUT_SECS))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());

        Self {
            server_url: server_url.into(),
            client,
            cached_tier_code: AtomicU8::new(TIER_CODE_COMMUNITY),
        }
    }

    pub fn cached_tier(&self) -> LicenseTier {
        if self.cached_tier_code.load(Ordering::SeqCst) == TIER_CODE_PRO {
            LicenseTier::Pro
        } else {
            LicenseTier::Community
        }
    }

    pub fn set_cached_tier(&self, tier: LicenseTier) {
        let code = match tier {
            LicenseTier::Community => TIER_CODE_COMMUNITY,
            LicenseTier::Pro => TIER_CODE_PRO,
        };
        self.cached_tier_code.store(code, Ordering::SeqCst);
    }

    pub fn get_active_tier(&self) -> LicenseTier {
        if self.cached_tier().is_pro() {
            return LicenseTier::Pro;
        }

        let current_machine_id = crate::machine_id::get_hardware_machine_id();
        let secret = crate::crypto::get_license_secret();

        if let Ok(token) = std::env::var(ENV_HAETAE_LICENSE_TOKEN) {
            if !secret.is_empty() {
                if let Ok(payload) = crate::crypto::verify_license_token(&token, None, &secret) {
                    if payload.tier == "PRO" {
                        self.set_cached_tier(LicenseTier::Pro);
                        return LicenseTier::Pro;
                    }
                }
            } else if let Ok(payload) = crate::crypto::parse_license_token_payload(&token) {
                let parts: Vec<&str> = token.split('.').collect();
                let sig_valid = parts.len() == TOKEN_PARTS_COUNT
                    && !parts[1].is_empty()
                    && crate::crypto::hex_decode(parts[1]).is_ok();

                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_secs())
                    .unwrap_or(0);

                if payload.tier == "PRO"
                    && sig_valid
                    && now < payload.expires_at
                    && (payload.machine_id == current_machine_id || payload.machine_id.is_empty())
                {
                    self.set_cached_tier(LicenseTier::Pro);
                    return LicenseTier::Pro;
                }
            }
        }

        if let Some(stored) = load_stored_license() {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0);

            let within_grace_period =
                now.saturating_sub(stored.last_verified_at) <= OFFLINE_GRACE_PERIOD_SECS;

            if !secret.is_empty() {
                if let Ok(payload) = crate::crypto::verify_license_token(
                    &stored.token,
                    Some(&stored.license_key),
                    &secret,
                ) {
                    if payload.tier == "PRO"
                        && within_grace_period
                        && (payload.machine_id == current_machine_id
                            || payload.machine_id.is_empty())
                    {
                        self.set_cached_tier(LicenseTier::Pro);
                        return LicenseTier::Pro;
                    }
                }
            } else if let Ok(payload) = crate::crypto::parse_license_token_payload(&stored.token) {
                let parts: Vec<&str> = stored.token.split('.').collect();
                let sig_valid = parts.len() == TOKEN_PARTS_COUNT
                    && !parts[1].is_empty()
                    && crate::crypto::hex_decode(parts[1]).is_ok();

                if payload.tier == "PRO"
                    && sig_valid
                    && now < payload.expires_at
                    && within_grace_period
                    && (payload.machine_id == current_machine_id || payload.machine_id.is_empty())
                {
                    self.set_cached_tier(LicenseTier::Pro);
                    return LicenseTier::Pro;
                }
            }
        }

        LicenseTier::Community
    }

    pub async fn verify_license(
        &self,
        license_key: &str,
        machine_id: &str,
    ) -> Result<LicenseResponse, AuthError> {
        let base = self.server_url.trim_end_matches('/');
        let url = format!("{}{}", base, ROUTE_AUTH_VERIFY);
        let req_body = LicenseRequest {
            license_key: license_key.to_string(),
            machine_id: machine_id.to_string(),
        };

        let response = self
            .client
            .post(&url)
            .json(&req_body)
            .send()
            .await
            .map_err(|e| AuthError::Network(e.to_string()))?;

        if !response.status().is_success() {
            return Err(AuthError::ServerError(response.status().as_u16()));
        }

        let resp_data: LicenseResponse = response
            .json()
            .await
            .map_err(|e| AuthError::Serialization(e.to_string()))?;

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        if resp_data.valid {
            self.set_cached_tier(resp_data.tier);
            let stored = StoredLicense {
                license_key: license_key.to_string(),
                token: resp_data.token.clone().unwrap_or_default(),
                tier: resp_data.tier,
                last_verified_at: now,
                expires_at: resp_data.expires_at,
            };
            let _ = save_stored_license(&stored);
        } else {
            self.set_cached_tier(LicenseTier::Community);
        }

        Ok(resp_data)
    }

    pub async fn verify_license_auto(
        &self,
        license_key: &str,
    ) -> Result<LicenseResponse, AuthError> {
        let machine_id = get_hardware_machine_id();
        self.verify_license(license_key, &machine_id).await
    }
}
