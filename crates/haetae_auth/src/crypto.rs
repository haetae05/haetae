use crate::client::{AuthError, LicenseTier};
use haetae_core::config::{
    ENV_HAETAE_LICENSE_SECRET, HEX_CHARS_PER_BYTE, HEX_RADIX, LICENSE_KEY_PREFIX, TOKEN_PARTS_COUNT,
};
use ring::hmac::{sign, verify, Key, HMAC_SHA256};

const LICENSE_PAYLOAD_SEGMENTS_COUNT: usize = 4;
const LICENSE_PAYLOAD_TIER_INDEX: usize = 2;
const LICENSE_PAYLOAD_EXPIRES_INDEX: usize = 3;
const LICENSE_KEY_SIG_BYTES_LEN: usize = 8;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct LicenseTokenPayload {
    pub license_key: String,
    pub machine_id: String,
    pub tier: String,
    pub expires_at: u64,
}

pub fn get_license_secret() -> String {
    std::env::var(ENV_HAETAE_LICENSE_SECRET).unwrap_or_default()
}

pub fn hex_decode(hex_str: &str) -> Result<Vec<u8>, AuthError> {
    if !hex_str.len().is_multiple_of(HEX_CHARS_PER_BYTE) {
        return Err(AuthError::InvalidLicense);
    }
    let mut bytes = Vec::new();
    let chars: Vec<char> = hex_str.chars().collect();
    let mut idx = 0;
    while idx < chars.len() {
        let chunk: String = chars[idx..idx + HEX_CHARS_PER_BYTE].iter().collect();
        let b = u8::from_str_radix(&chunk, HEX_RADIX).map_err(|_| AuthError::InvalidLicense)?;
        bytes.push(b);
        idx += HEX_CHARS_PER_BYTE;
    }
    Ok(bytes)
}

pub fn generate_license_token(
    license_key: &str,
    machine_id: &str,
    tier: LicenseTier,
    expires_at: u64,
    secret: &str,
) -> String {
    let tier_str = match tier {
        LicenseTier::Pro => "PRO",
        LicenseTier::Community => "COMMUNITY",
    };
    let payload_str = format!("{}:{}:{}:{}", license_key, machine_id, tier_str, expires_at);
    let payload_hex = payload_str
        .as_bytes()
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();

    let key = Key::new(HMAC_SHA256, secret.as_bytes());
    let tag = sign(&key, payload_str.as_bytes());
    let sig_hex = tag
        .as_ref()
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();

    format!("{}.{}", payload_hex, sig_hex)
}

pub fn parse_license_token_payload(token: &str) -> Result<LicenseTokenPayload, AuthError> {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != TOKEN_PARTS_COUNT {
        return Err(AuthError::InvalidLicense);
    }
    let payload_hex = parts[0];
    let sig_hex = parts[1];
    if sig_hex.is_empty() {
        return Err(AuthError::InvalidLicense);
    }

    let payload_bytes = hex_decode(payload_hex)?;
    let payload_str = String::from_utf8(payload_bytes).map_err(|_| AuthError::InvalidLicense)?;
    let segs: Vec<&str> = payload_str.split(':').collect();
    if segs.len() != LICENSE_PAYLOAD_SEGMENTS_COUNT {
        return Err(AuthError::InvalidLicense);
    }

    let lic_key = segs[0].to_string();
    let machine_id = segs[1].to_string();
    let tier_str = segs[LICENSE_PAYLOAD_TIER_INDEX].to_string();
    let expires_at = segs[LICENSE_PAYLOAD_EXPIRES_INDEX]
        .parse::<u64>()
        .map_err(|_| AuthError::InvalidLicense)?;

    Ok(LicenseTokenPayload {
        license_key: lic_key,
        machine_id,
        tier: tier_str,
        expires_at,
    })
}

pub fn verify_license_token(
    token: &str,
    expected_key: Option<&str>,
    secret: &str,
) -> Result<LicenseTokenPayload, AuthError> {
    if secret.trim().is_empty() {
        return Err(AuthError::InvalidLicense);
    }

    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != TOKEN_PARTS_COUNT {
        return Err(AuthError::InvalidLicense);
    }
    let payload_hex = parts[0];
    let sig_hex = parts[1];

    let payload_bytes = hex_decode(payload_hex)?;
    let sig_bytes = hex_decode(sig_hex)?;

    let key = Key::new(HMAC_SHA256, secret.as_bytes());
    verify(&key, &payload_bytes, &sig_bytes).map_err(|_| AuthError::InvalidLicense)?;

    let payload = parse_license_token_payload(token)?;

    if let Some(exp_key) = expected_key {
        if payload.license_key != exp_key {
            return Err(AuthError::InvalidLicense);
        }
    }

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    if now > payload.expires_at {
        return Err(AuthError::InvalidLicense);
    }

    Ok(payload)
}

pub fn generate_signed_license_key(key_id: &str, tier: LicenseTier, secret: &str) -> String {
    let tier_str = match tier {
        LicenseTier::Pro => "PRO",
        LicenseTier::Community => "COMMUNITY",
    };
    let payload = format!("{}:{}", tier_str, key_id);
    let key = Key::new(HMAC_SHA256, secret.as_bytes());
    let tag = sign(&key, payload.as_bytes());
    let sig_hex = tag
        .as_ref()
        .iter()
        .take(LICENSE_KEY_SIG_BYTES_LEN)
        .map(|b| format!("{:02x}", b))
        .collect::<String>();
    format!("HAETAE-{}-{}-{}", tier_str, key_id, sig_hex)
}

pub fn verify_license_key(key_str: &str, secret: &str) -> Result<LicenseTier, AuthError> {
    if secret.trim().is_empty() {
        return Err(AuthError::InvalidLicense);
    }

    let trimmed = key_str.trim();
    let rest = trimmed
        .strip_prefix(LICENSE_KEY_PREFIX)
        .ok_or(AuthError::InvalidLicense)?;
    let first_dash = rest.find('-').ok_or(AuthError::InvalidLicense)?;
    let tier_str = &rest[..first_dash];
    let remainder = &rest[first_dash + 1..];
    let last_dash = remainder.rfind('-').ok_or(AuthError::InvalidLicense)?;
    let key_id = &remainder[..last_dash];
    let sig_hex = &remainder[last_dash + 1..];

    if key_id.is_empty() || sig_hex.is_empty() {
        return Err(AuthError::InvalidLicense);
    }

    let tier = match tier_str {
        "PRO" => LicenseTier::Pro,
        "COMMUNITY" => LicenseTier::Community,
        _ => return Err(AuthError::InvalidLicense),
    };

    let payload = format!("{}:{}", tier_str, key_id);
    let key = Key::new(HMAC_SHA256, secret.as_bytes());
    let tag = sign(&key, payload.as_bytes());
    let expected_sig = tag
        .as_ref()
        .iter()
        .take(LICENSE_KEY_SIG_BYTES_LEN)
        .map(|b| format!("{:02x}", b))
        .collect::<String>();

    if sig_hex.to_lowercase() != expected_sig.to_lowercase() {
        return Err(AuthError::InvalidLicense);
    }

    Ok(tier)
}
