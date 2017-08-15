
use std::env;

pub fn get_proxies() -> Result<Vec<Url>, ProxyConfigError> {
    let vars = env::vars().collect();
    let mut result = vec![];
    for (key, value) in vars {
        if key.to_lowercase().ends_with("_proxy") {
            let scheme = key[..6];
            
        }
    }
    Ok(result)
}
