extern crate winreg;

use super::*;
use self::winreg::{RegKey, RegValue};
use self::winreg::enums::*;

const REG_SETTINGS: &str = r"Software\Microsoft\Windows\CurrentVersion\Internet Settings";
const REG_CONNECTIONS: &str =
    r"Software\Microsoft\Windows\CurrentVersion\Internet Settings\Connections";

enum AutoconfigType {
    Pac,
    Wpad,
    None,
}

fn proxy_autoconfig_type() -> AutoconfigType {
    if let Ok(key) = RegKey::predef(HKEY_CURRENT_USER).open_subkey_with_flags(REG_CONNECTIONS, KEY_READ) {
        match key.get_raw_value("DefaultConnectionSettings") {
            Ok(RegValue { ref bytes, .. }) if bytes.len() > 8 => {
                if (bytes[8] & (1 << 2)) == bytes[8] {
                    return AutoconfigType::Pac;
                } else if (bytes[8] & (1 << 3)) == bytes[8] {
                    return AutoconfigType::Wpad;
                }
            },
            _ => {},
        }
    }
    AutoconfigType::None
}

pub fn get_proxy_strings() -> Result<Vec<String>, ProxyConfigError> {
    match proxy_autoconfig_type() {
        AutoconfigType::Pac => return Err(ProxyTypeNotSupportedError("PAC")),
        AutoconfigType::Wpad => return Err(ProxyTypeNotSupportedError("WPAD")),
        AutoconfigType::None => {},
    };

    if let Ok(key) = RegKey::predef(HKEY_CURRENT_USER).open_subkey_with_flags(REG_SETTINGS, KEY_READ) {
        if key.get_value("ProxyEnabled").unwrap_or(0u32) != 0 {
            if let Ok(config) = key.get_value("ProxyServer") {
                let config: String = config;

                // There are two types of ProxyServer values:
                // - 1.2.3.4:8080
                // - http=1.2.3.4:8080;https=1.2.3.4:8080;...
                if config.contains(";") {
                    let mut result = Vec::new();
                    for proxy in config.split(";") {
                        let split: Vec<&str> = proxy.split("=").collect();
                        if split.len() != 2 { 
                            return Err(InvalidConfigError("invalid proxy list in Registry"));
                        }
                        result.push(format!("{}://{}", split[0], split[1]));
                    }
                    return Ok(result);
                } else {
                    return Ok(vec![config]);
                }
            }
        }
    }

    Err(NoProxyConfiguredError)
}
