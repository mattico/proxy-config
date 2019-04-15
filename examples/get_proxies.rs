extern crate proxy_cfg;
extern crate url;

use proxy_cfg::*;
use url::Url;

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() == 0 {
        match get_proxy_config() {
            Ok(ProxyConfig { proxies, .. }) => {
                for (_, p) in proxies {
                    println!("{}", p.to_string());
                }
            },
            Err(e) => {
                println!("Error getting proxies: {:?}", e);
                process::exit(1);
            },
        };
    } else {
        for arg in args {
            match get_proxy_for_url(Url::parse(&arg).unwrap()) {
                Ok(proxy) => println!("{} : {}", arg, proxy),
                Err(ProxyConfigError::NoProxyNeededError) => println!("No proxy needed for URL: '{}'", arg),
                Err(e) => {
                    println!("Error getting proxy for URL '{}': {}", arg, e);
                    process::exit(1);
                },
            }
        }
    }
}


