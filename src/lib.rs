use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProxyConfig {
    auto_detect: Option<bool>,
    autoconfig_url: Option<String>,
    proxy_url: Option<String>,
}

pub fn get_proxy() -> Result<ProxyConfig, ProxyConfigError> {
    plat::get_proxy()
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ProxyConfigError {
    NoConfigError,
    OsError,
    NotSupportedError,
}

impl fmt::Display for ProxyConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ProxyConfigError::NoConfigError => write!(f, "No proxy configuration found"),
            ProxyConfigError::OsError => 
                write!(f, "Error getting proxy configuration from the Operating System"),
            ProxyConfigError::NotSupportedError => {
                write!(f, "Can not read proxy configuration on this platform")
            }
        }
    }
}

#[cfg(windows)]
mod plat {
    extern crate winapi;
    extern crate kernel32;
    use self::winapi::*;
    use super::*;

    #[link(name = "winhttp")]
    extern "system" {
        fn WinHttpGetIEProxyConfigForCurrentUser(
            pProxyConfig: *mut WINHTTP_CURRENT_USER_IE_PROXY_CONFIG,
        ) -> BOOL;
    }

    unsafe fn from_wide_ptr(ptr: *const u16) -> Option<String> {
        use std::ffi::OsString;
        use std::os::windows::ffi::OsStringExt;
        use std::slice;

        if ptr.is_null() {
            return None;
        }

        let mut len = 0usize;
        while *ptr.offset(len as isize) != 0 {
            len += 1;
        }

        let slice = slice::from_raw_parts(ptr, len);
        Some(OsString::from_wide(slice).to_string_lossy().into_owned())
    }

    unsafe fn free_str(ptr: *mut u16) {
        if !ptr.is_null() {
            kernel32::GlobalFree(ptr as *mut _);
        }
    }

    pub fn get_proxy() -> Result<ProxyConfig, ProxyConfigError> {
        let mut config: WINHTTP_CURRENT_USER_IE_PROXY_CONFIG = unsafe { 
            ::std::mem::uninitialized() 
        };
        let result = unsafe { WinHttpGetIEProxyConfigForCurrentUser(&mut config as *mut _) };
        if result == TRUE {
            let autodetect = if config.fAutoDetect == TRUE {
                true
            } else {
                false
            };
            let autoconfig;
            let proxyurl;

            unsafe {
                autoconfig = from_wide_ptr(config.lpszAutoConfigUrl);
                proxyurl = from_wide_ptr(config.lpszProxy);

                free_str(config.lpszAutoConfigUrl);
                free_str(config.lpszProxy);
                free_str(config.lpszProxyBypass);
            }

            Ok(ProxyConfig {
                auto_detect: Some(autodetect),
                autoconfig_url: autoconfig,
                proxy_url: proxyurl,
            })
        } else {
            let err_code = unsafe { kernel32::GetLastError() };

            Err(match err_code {
                ERROR_FILE_NOT_FOUND => ProxyConfigError::NoConfigError,
                _ => ProxyConfigError::OsError,
            })
        }
    }
}

#[cfg(not(windows))]
mod plat {
    pub fn get_proxy() -> Result<ProxyConfig, ProxyConfigError> {
        Err(ProxyConfigError::NotSupportedError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_proxy() {
        let proxy_config = get_proxy();
        if proxy_config != Err(ProxyConfigError::NotSupportedError) {
            assert!(proxy_config.is_ok());
        }
    }
}
