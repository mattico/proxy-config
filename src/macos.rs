use plist::Plist;
use std::{
    fs::File,
    io::Read,
    collections::HashMap,
};

use super::*;

#[allow(unused_assignments)]
pub(crate) fn get_proxy_config() -> Result<ProxyConfig> {
    let mut xml_data = String::new();

    if File::open("/Library/Preferences/SystemConfiguration/preferences.plist").ok()
        .and_then(|mut file| file.read_to_string(&mut xml_data).ok())
        == Some(0) {
        // Failed to read preferences.plist
        return Err(OsError)
    }

    if let Some(Plist::Dict(decoded_data)) = Plist::from_xml_reader(&mut xml_data.as_bytes()).ok() {
        /* Decoded data will have the following structure:
        Dict({
            "9749BB80-B540-4B2D-8EC8-32CA31496C89": Dict({
                "IPv6": Dict({
                            "ConfigMethod": String("Automatic")
                        }),
                "Proxies": Dict({
                    "ExceptionsList": Array([
                                String("*.local"),
                                String("169.254/16")
                            ]),
                    "FTPPassive": Integer(1)
                }),
                "IPv4": Dict ({
                        "ConfigMethod": String("DHCP")
                    }),
                "SMB": Dict({}),
                "DNS": Dict({}),
                "Interface":  Dict({
                        "DeviceName": String("en6"),
                        "Hardware": String("Ethernet"),
                        "Type": String("Ethernet"),
                        "UserDefinedName": String("USB 10/100/1000 LAN")
                    }),
                "UserDefinedName": String("USB 10/100/1000 LAN")
            }),
            "...": Dict({...})
        })
        */

        if let Some(Plist::Dict(network_services)) = decoded_data.get("NetworkServices") {
            let mut proxies = HashMap::new();
            let mut whitelist = Vec::new();

            for interface in network_services.keys() {
                if let Some(Plist::Dict(interface)) = network_services.get(interface) {
                    if let Some(Plist::Dict(proxy)) = interface.get("Proxies") {
                        let mut var = "";
                        let mut scheme = "";
                        for entry in proxy.keys() {
                            match entry.as_ref(){
                                "HTTPProxy" => {
                                    var = "HTTP";
                                    scheme = "http";
                                }
                                "HTTPSProxy" => {
                                    var = "HTTPS";
                                    scheme = "https"
                                }
                                "SOCKSProxy" => {
                                    var = "SOCKS";
                                    scheme = "http"
                                }
                                "FTPProxy" => {
                                    var = "FTP";
                                    scheme = "http"
                                }
                                _ => {continue}
                            }
                            if proxy.get(&format!("{}{}",var,"Enable")) == Some(&Plist::Integer(1)) {
                                proxies.insert(
                                    var.to_lowercase(),
                                    util::parse_addr_default_scheme(
                                        scheme,
                                        &format!(
                                            "{}:{}",
                                            get_string(proxy.get(&format!("{}{}", var, "Proxy"))),
                                            get_int(proxy.get(&format!("{}{}", var, "Port")))
                                        )
                                    )?
                                );
                            } else {
                                continue
                            }
                        }
                        if let Some(Plist::Array(exceptions)) = proxy.get("ExceptionsList") {
                            for exception in exceptions {
                                whitelist.push(get_string(Some(exception)));
                            }
                        }
                    }
                }
            }

            return Ok(ProxyConfig {
                proxies,
                whitelist,  // TODO: no way of knowing for which proxy...
                ..Default::default()
            });
        }
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

//TODO: impl test?
