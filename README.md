# proxy-config
[![Appveyor Build status](https://ci.appveyor.com/api/projects/status/uip8grlodr0y4q8c/branch/master?svg=true)](https://ci.appveyor.com/project/mattico/proxy-config/branch/master)
[![Travis Build Status](https://travis-ci.org/mattico/proxy-config.svg?branch=master)](https://travis-ci.org/mattico/proxy-config)
[![Crates.io](https://img.shields.io/crates/v/proxy-config.svg)](https://crates.io/crates/proxy-config)
[![Documentation](https://docs.rs/proxy_config/badge.svg)](https://docs.rs/proxy_config)

A Rust library to get proxy configuration from the OS.

## Usage

```Rust
extern crate proxy_config;

if let Ok(proxy) = proxy_config::get_proxy_for_url(&url) {
    // use proxy to connect...
}
```

## License

This project is provided under the terms of the Apache License 2.0 or the MIT License, at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
