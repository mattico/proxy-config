extern crate url;

use std::fmt;

use url::Url;

#[cfg(windows)]
mod windows;

#[cfg(feature = "env")]
mod env;

type ProxyFn = fn() -> Result<Vec<String>, ProxyConfigError>;

const METHODS: &[&ProxyFn] = &[
    #[cfg(feature = "env")]
    &(env::get_proxy_strings as ProxyFn),
    #[cfg(windows)]
    &(windows::get_proxy_strings as ProxyFn),
];

/// Returns a vector of URLs for the proxies configured by the system
pub fn get_proxies() -> Result<Vec<Url>, ProxyConfigError> {
    let mut last_err = PlatformNotSupportedError;
    for get_proxy_strings in METHODS {
        match get_proxy_strings() {
            Ok(strings) => {
                let mut result = vec![];
                for string in strings {
                    if let Ok(url) = Url::parse(&string) {
                        result.push(url);
                    } else {
                        return Err(InvalidConfigError("unable to parse proxy URL"));
                    }
                }
                return Ok(result);
            },
            Err(e) => last_err = e,
        }
    }
    Err(last_err)
}

/// Returns the proxy to use for the given URL
pub fn get_proxy_for_url(url: Url) -> Result<Url, ProxyConfigError> {
    // TODO: cache get_proxies result?
    match get_proxies() {
        Ok(proxies) => {
            for proxy in proxies {
                if proxy.scheme() == url.scheme() {
                    return Ok(proxy);
                }
            }
            return Err(NoProxyForSchemeError(url.scheme().to_string()));
        },
        Err(e) => Err(e),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProxyConfigError {
    InvalidConfigError(&'static str),
    NoProxyConfiguredError,
    NoProxyForSchemeError(String),
    OsError,
    PlatformNotSupportedError,
    ProxyTypeNotSupportedError(&'static str),
}
use ProxyConfigError::*;

impl fmt::Display for ProxyConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InvalidConfigError(s) => write!(f, "Proxy configuration invalid: {}", s),
            NoProxyConfiguredError => write!(f, "No proxy configuration found"),
            NoProxyForSchemeError(ref s) => write!(f, "No proxy found for scheme: {}", s),
            OsError => 
                write!(f, "Error getting proxy configuration from the Operating System"),
            PlatformNotSupportedError => {
                write!(f, "Can not read proxy configuration on this platform")
            },
            ProxyTypeNotSupportedError(s) => {
                write!(f, "Proxy type not supported: {}", s)
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_proxy() {
        let proxy_config = get_proxies();
        assert!(proxy_config.is_ok());
    }
}
