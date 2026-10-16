use haetae_core::config::{ENV_HAETAE_MACHINE_ID, MAC_ADDRESS_STR_LEN, SHA256_HEX_STRING_LEN};
use ring::digest::{digest, SHA256};
use std::env;

pub const DEFAULT_FALLBACK_HOST: &str = "haetae-node";
pub const DEFAULT_FALLBACK_USER: &str = "anonymous";
pub const DEFAULT_FALLBACK_UUID: &str = "00000000-0000-0000-0000-000000000000";
pub const DEFAULT_FALLBACK_MAC: &str = "00:00:00:00:00:00";
pub const MACHINE_ID_NAMESPACE: &str = "haetae-machine-digest-v1";

#[cfg(target_os = "windows")]
fn retrieve_machine_uuid() -> Option<String> {
    use haetae_core::config::WIN_CREATE_NO_WINDOW_FLAG;
    use std::os::windows::process::CommandExt;
    use std::process::Command;

    let output = Command::new("reg")
        .args([
            "query",
            "HKLM\\SOFTWARE\\Microsoft\\Cryptography",
            "/v",
            "MachineGuid",
        ])
        .creation_flags(WIN_CREATE_NO_WINDOW_FLAG)
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let text = String::from_utf8_lossy(&output.stdout);
    for line in text.lines() {
        if line.contains("MachineGuid") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if let Some(val) = parts.last() {
                if !val.is_empty() {
                    return Some((*val).to_string());
                }
            }
        }
    }
    None
}

#[cfg(not(target_os = "windows"))]
fn retrieve_machine_uuid() -> Option<String> {
    let candidate_paths = [
        "/etc/machine-id",
        "/var/lib/dbus/machine-id",
        "/sys/class/dmi/id/product_uuid",
    ];
    for path in candidate_paths {
        if let Ok(content) = std::fs::read_to_string(path) {
            let trimmed = content.trim();
            if !trimmed.is_empty() {
                return Some(trimmed.to_string());
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        if let Ok(output) = Command::new("ioreg")
            .args(["-rd1", "-c", "IOPlatformExpertDevice"])
            .output()
        {
            if output.status.success() {
                let text = String::from_utf8_lossy(&output.stdout);
                for line in text.lines() {
                    if line.contains("IOPlatformUUID") {
                        if let Some(pos) = line.find('"') {
                            let rest = &line[pos + 1..];
                            if let Some(end_pos) = rest.rfind('"') {
                                let guid = &rest[..end_pos];
                                if !guid.is_empty() {
                                    return Some(guid.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    None
}

#[cfg(target_os = "windows")]
fn retrieve_mac_address() -> Option<String> {
    use haetae_core::config::WIN_CREATE_NO_WINDOW_FLAG;
    use std::os::windows::process::CommandExt;
    use std::process::Command;

    if let Ok(output) = Command::new("getmac")
        .args(["/fo", "csv", "/nh"])
        .creation_flags(WIN_CREATE_NO_WINDOW_FLAG)
        .output()
    {
        if output.status.success() {
            let text = String::from_utf8_lossy(&output.stdout);
            for line in text.lines() {
                let trimmed = line.trim().trim_matches('"');
                if let Some(first_field) = trimmed.split('"').next() {
                    let cleaned = first_field.trim();
                    if cleaned.len() >= MAC_ADDRESS_STR_LEN && cleaned.contains('-') {
                        return Some(cleaned.to_string());
                    }
                }
            }
        }
    }
    None
}

#[cfg(target_os = "linux")]
fn retrieve_mac_address() -> Option<String> {
    let net_paths = [
        "/sys/class/net/eth0/address",
        "/sys/class/net/en0/address",
        "/sys/class/net/wlan0/address",
        "/sys/class/net/ens3/address",
    ];
    for path in net_paths {
        if let Ok(mac) = std::fs::read_to_string(path) {
            let trimmed = mac.trim();
            if !trimmed.is_empty() && trimmed != DEFAULT_FALLBACK_MAC {
                return Some(trimmed.to_string());
            }
        }
    }
    None
}

#[cfg(not(any(target_os = "windows", target_os = "linux")))]
fn retrieve_mac_address() -> Option<String> {
    None
}

fn retrieve_hostname() -> String {
    env::var("HOSTNAME")
        .or_else(|_| env::var("COMPUTERNAME"))
        .unwrap_or_else(|_| DEFAULT_FALLBACK_HOST.to_string())
}

fn retrieve_username() -> String {
    env::var("USER")
        .or_else(|_| env::var("USERNAME"))
        .or_else(|_| env::var("LOGNAME"))
        .unwrap_or_else(|_| DEFAULT_FALLBACK_USER.to_string())
}

pub fn compute_sha256_hex(data: &[u8]) -> String {
    let digest_val = digest(&SHA256, data);
    digest_val
        .as_ref()
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>()
}

pub fn get_hardware_machine_id() -> String {
    let override_opt = env::var(ENV_HAETAE_MACHINE_ID).ok();
    get_hardware_machine_id_with_override(override_opt.as_deref())
}

pub fn get_hardware_machine_id_with_override(override_id: Option<&str>) -> String {
    if let Some(val) = override_id {
        let trimmed = val.trim();
        if trimmed.len() == SHA256_HEX_STRING_LEN && trimmed.chars().all(|c| c.is_ascii_hexdigit())
        {
            return trimmed.to_ascii_lowercase();
        } else if !trimmed.is_empty() {
            return compute_sha256_hex(trimmed.as_bytes());
        }
    }

    let hostname = retrieve_hostname();
    let username = retrieve_username();
    let machine_uuid = retrieve_machine_uuid()
        .or_else(|| env::var("PROCESSOR_IDENTIFIER").ok())
        .unwrap_or_else(|| DEFAULT_FALLBACK_UUID.to_string());
    let mac = retrieve_mac_address().unwrap_or_else(|| DEFAULT_FALLBACK_MAC.to_string());

    let payload = format!(
        "{}:{}:{}:{}:{}:{}",
        MACHINE_ID_NAMESPACE,
        hostname.trim(),
        username.trim(),
        machine_uuid.trim(),
        mac.trim(),
        env::consts::ARCH
    );

    compute_sha256_hex(payload.as_bytes())
}
