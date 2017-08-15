use std::error::Error;
use std::fmt;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum ProxyConfigError {
    InvalidConfigError,
    NoProxyConfiguredError,
    NoProxyForSchemeError(String),
    NoProxyNeededError,
    OsError,
    PlatformNotSupportedError,
    ProxyTypeNotSupportedError(String),
}
use ProxyConfigError::*;
pub type Result<T> = ::std::result::Result<T, ProxyConfigError>;

impl Error for ProxyConfigError {
    fn description(&self) -> &str {
        match *self {
            InvalidConfigError => "invalid proxy configuration",
            NoProxyConfiguredError => "no proxy configuration found",
            NoProxyForSchemeError(_) => "no proxy found for scheme",
            NoProxyNeededError => "no proxy needed for the given URL",
            OsError => "error getting proxy configuration from the Operating System",
            PlatformNotSupportedError => "can not read proxy configuration on this platform",
            ProxyTypeNotSupportedError(_) => "proxy type not supported",
        }
    }
}

impl fmt::Display for ProxyConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NoProxyForSchemeError(ref scheme) => write!(f, "no proxy found for scheme: '{}'", scheme),
            ProxyTypeNotSupportedError(ref proxy_type) => write!(f, "proxy type not supported: '{}'", proxy_type),
            _ => self.description().fmt(f),
        }        
    }
}

impl From<::url::ParseError> for ProxyConfigError {
    fn from(_error: ::url::ParseError) -> Self {
        InvalidConfigError
    }
}
