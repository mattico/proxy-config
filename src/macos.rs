extern crate serde_json;

use plist::Plist;
use std::{
    fs::File,
    collections::HashMap,
};

use super::*;
#[derive(Debug)]
pub struct ProxyConfigInternal {
    pub proxy: Url,
    pub port: i64,
    pub protocol: String,
    pub interface: String,
    pub whitelist: String,
}

pub(crate) fn get_proxy_config_internal() -> Result<Vec<ProxyConfigInternal>> {

    let plist = File::open("/Library/Preferences/SystemConfiguration/preferences.plist").ok()
        .and_then(|file| Plist::read(file).ok()).ok_or(OsError)?;

    if let Some(Plist::Dictionary(network_services)) = plist.as_dictionary()
        .and_then(|decoded_data| decoded_data.get("NetworkServices")) {

        let mut proxies = Vec::new();

        // Extract proxy settings for all network interfaces.
        for (_k,v) in network_services.iter() {

            let proxy = v.as_dictionary().ok_or(InvalidConfigError)?
                .get("Proxies").ok_or(InvalidConfigError)?
                .as_dictionary().ok_or(InvalidConfigError)?;

            for entry in proxy.keys() {
                if entry.contains("Proxy") {
                    // Ex: entry = "HTTPSProxy".
                    let protocol = entry.replace("Proxy","");
                    let scheme;
                    match protocol.as_ref() {
                        "HTTPS" => {
                            scheme = "https"
                        },
                        _ => {
                            scheme = "http"
                        }
                    };
                    if proxy.get(&format!("{}{}",protocol,"Enable"))
                        == Some(&Plist::Integer(1)) {
                        let mut interface = String::new();
                        let mut whitelist = Vec::new();

                        if let Some(Plist::Array(exceptions)) = proxy.get("ExceptionsList") {
                            // Proxy exceptions can be different for different network interfaces on MacOs.
                            if let Some(Plist::String(user_defined_name)) = v
                                .as_dictionary().ok_or(InvalidConfigError)?
                                .get("UserDefinedName"){
                                interface = user_defined_name.to_string();
                                for exception in exceptions {
                                    whitelist.push(
                                        exception.as_string().ok_or(InvalidConfigError)?
                                    )
                                }
                            }
                        }

                        proxies.push(
                            ProxyConfigInternal {
                                proxy: util::parse_addr_default_scheme(
                                    scheme,
                                    &format!(
                                        "{}:{}",
                                        proxy.get(entry)
                                            .ok_or(InvalidConfigError)?
                                            .as_string()
                                            .ok_or(InvalidConfigError)?,
                                        proxy.get(&format!("{}{}", protocol, "Port"))
                                            .ok_or(InvalidConfigError)?
                                            .as_integer()
                                            .ok_or(InvalidConfigError)?
                                    )
                                )?,
                                port: proxy.get(&format!("{}{}", protocol, "Port"))
                                    .ok_or(InvalidConfigError)?
                                    .as_integer()
                                    .ok_or(InvalidConfigError)?,
                                protocol: protocol.to_lowercase(),
                                interface,
                                whitelist:serde_json::to_string(&whitelist).ok()
                                    .unwrap_or(String::new()),
                            }
                        );
                    } else {
                        // Proxy for protocol is not enabled.
                        continue
                    }
                }
            }
        }
        return Ok(proxies)
    }
    Err(NoProxyConfiguredError)
}

pub(crate) fn get_proxy_config() -> Result<ProxyConfig> {
    let proxy_configs = get_proxy_config_internal()?;
    let mut proxies = HashMap::new();
    let mut whitelist = Vec::new();
    for proxy_config in proxy_configs {
        proxies.insert(
            proxy_config.protocol,
            proxy_config.proxy
        );
        whitelist.push(proxy_config.whitelist);
    }
    return Ok(ProxyConfig {
        proxies,
        whitelist,
        ..Default::default()
    });
}