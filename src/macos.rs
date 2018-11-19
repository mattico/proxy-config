use plist::Plist;
use std::{
    fs::File,
    io::Read,
    collections::HashMap,
};

use super::*;

pub(crate) fn get_proxy_config() -> Result<ProxyConfig> {

    let plist = File::open("/Library/Preferences/SystemConfiguration/preferences.plist").ok()
        .and_then(|file| Plist::read(file).ok()).ok_or(OsError)?;

    if let Some(Plist::Dictionary(network_services)) = plist.as_dictionary()
        .and_then(|decoded_data| decoded_data.get("NetworkServices")) {

        let mut proxies = HashMap::new();
        let mut whitelist = Vec::new();

        for (_k,v) in network_services.iter() {

            let proxy = v.as_dictionary().ok_or(InvalidConfigError)?
                .get("Proxies").ok_or(InvalidConfigError)?
                .as_dictionary().ok_or(InvalidConfigError)?;

            for entry in proxy.keys() {
                if entry.contains("Proxy") {
                    // Ex: entry = "HTTPSProxy"
                    let protocol = entry.replace("Proxy","");
                    let scheme;
                    match protocol.to_lowercase().as_ref() {
                        "https" => {
                            scheme = "https"
                        },
                        _ => {
                            scheme = "http"
                        }
                    };
                    if proxy.get(&format!("{}{}",protocol,"Enable")) == Some(&Plist::Integer(1)) {
                        proxies.insert(
                            protocol.to_lowercase(),
                            util::parse_addr_default_scheme(
                                scheme,
                                &format!(
                                    "{}:{}",
                                    get_string(proxy.get(entry)),
                                    get_int(proxy.get(&format!("{}{}", protocol, "Port")))
                                )
                            )?
                        );
                    } else {
                        continue
                    }
                }
            }
            if let Some(Plist::Array(exceptions)) = proxy.get("ExceptionsList") {
                for exception in exceptions {
                    whitelist.push(get_string(Some(exception)));
                }
            }
        }
        return Ok(ProxyConfig {
            proxies,
            whitelist,  // TODO: no way of knowing for which proxy...
            ..Default::default()
        });
    }
    Err(NoProxyConfiguredError)
}

fn get_string(s:Option<&Plist>) -> String {
    match s {
        Some(Plist::String(v)) => v.to_owned(),
        _ => "".to_owned(),
    }
}

fn get_int(i:Option<&Plist>) -> i64 {
    match i {
        Some(Plist::Integer(v)) => *v,
        _ => -1,
    }
}