extern crate proxy_config;
extern crate url;
use url::Url;

pub fn main () {
    println!("{}",proxy_config::get_proxy_for_url(
        Url::parse("http://google.com").unwrap()).ok().unwrap().as_str()
    );
    match proxy_config::get_proxy_config() {
        Ok(proxy_config::ProxyConfig { proxies, whitelist: _, .. }) => {
            for p in proxies {
                println!("{:?}", p);
            }
        },
        Err(e) => {
            println!("Error getting proxies: {:?}", e);
        },
    };
}