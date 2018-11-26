extern crate proxy_config;
extern crate url;
use url::Url;

pub fn main () {
    match proxy_config::get_proxy_for_url(Url::parse("http://google.com").unwrap()).ok(){
        Some(a) => {
            println!("Proxy for google.com: {}", a);
        },
        None => {
            println!("No need for a proxy.");
        },
    };
    match proxy_config::get_proxy_config() {
        Ok(proxy_config::ProxyConfig { proxies, whitelist, .. }) => {
            for p in proxies {
                println!("Proxy: {:?}", p);
            }
            for e in whitelist {
                println!("Exceptions: {}", e);
            }
        },
        Err(e) => {
            println!("Error getting proxies: {:?}", e);
        },
    };
}