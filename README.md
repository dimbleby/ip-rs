# ip-rs #

A tiny crate providing the `IpAddr` enum, which can represent either an IPv4 or an IPv6 address.

[![Build Status](https://travis-ci.org/dimbleby/ip-rs.svg?branch=master)](https://travis-ci.org/dimbleby/ip-rs)
[![Build status](https://ci.appveyor.com/api/projects/status/ptcqa6x59vy82437/branch/master?svg=true)](https://ci.appveyor.com/project/dimbleby/ip-rs/branch/master)
[![crates.io](https://meritbadge.herokuapp.com/ip)](https://crates.io/crates/ip)

## Do not use this code! ##

As of Rust 1.7.0, the `std::net::IpAddr` is stabilized - and should be preferred.

## Documentation ##

API documentation is [here](https://docs.rs/ip).

## Installation ##

In `Cargo.toml`:

```toml
[dependencies]
ip = "*"
```

And in your crate root:

```rust
extern crate ip;
```
