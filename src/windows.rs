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
        // Format of DefaultConnectionSettings is a string of bytes
        // Only interested in byte 9 here which values mean:
        //  09 when only 'Automatically detect settings' is enabled 
        //  03 when only 'Use a proxy server for your LAN' is enabled
        //  0B when both are enabled
        //  05 when only 'Use automatic configuration script' is enabled
        //  0D when 'Automatically detect settings' and 'Use automatic configuration script' are enabled
        //  07 when 'Use a proxy server for your LAN' and 'Use automatic configuration script' are enabled
        //  0F when all the three are enabled. 
        //  01 when none of them are enabled. 
        // Source https://superuser.com/questions/419696/in-windows-7-how-to-change-proxy-settings-from-command-line
        
        match key.get_raw_value("DefaultConnectionSettings") {
            Ok(RegValue { ref bytes, .. }) if bytes.len() > 8 => {
                if (bytes[8] & (1 << 2)) == (1 << 2) {
                    return AutoconfigType::Pac;
                } else if (bytes[8] & (1 << 3)) == (1 << 3) {
                    return AutoconfigType::Wpad;
                }
            },
            _ => {},
        }
    }
    AutoconfigType::None
}

pub(crate) fn get_proxy_config() -> Result<ProxyConfig> {
    match proxy_autoconfig_type() {
        AutoconfigType::Pac => return Err(ProxyTypeNotSupportedError("PAC".into())),
        AutoconfigType::Wpad => return Err(ProxyTypeNotSupportedError("WPAD".into())),
        AutoconfigType::None => {},
    };

    if let Ok(key) = RegKey::predef(HKEY_CURRENT_USER).open_subkey_with_flags(REG_SETTINGS, KEY_READ) {
        if key.get_value("ProxyEnable").unwrap_or(0u32) != 0 {
            if let Ok(config) = key.get_value("ProxyServer") {
                let config: String = config;
                let mut proxies = HashMap::new();
                let mut whitelist = Vec::new();

                // There are two types of ProxyServer values:
                // - 1.2.3.4:8080
                // - http=1.2.3.4:8080;https=1.2.3.4:8080;...
                if config.contains(";") {
                    for proxy in config.split(";").map(|s| s.trim()) {
                        let split: Vec<&str> = proxy.split("=").collect();
                        if split.len() != 2 { 
                            return Err(InvalidConfigError);
                        }
                        proxies.insert(split[0].into(), util::parse_addr_default_scheme(split[0], split[1])?);                     
                    }
                } else {
                    proxies.insert("http".into(), util::parse_addr_default_scheme("http", &config)?);
                }

                if let Ok(ignore) = key.get_value("ProxyOverride") {
                    let ignore: String = ignore;

                    for url in ignore.split(";").map(|s| s.trim()) {
                        if url.len() > 0 {
                            whitelist.push(url.into());
                        }
                    }
                }

                return Ok(ProxyConfig {
                    proxies,
                    whitelist,
                    ..Default::default()
                });
            }
        }
    }

    Err(NoProxyConfiguredError)
}
