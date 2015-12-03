# ip-rs #

A tiny crate providing the `IpAddr` enum, which can represent either an IPv4 or an IPv6 address.

[![Build Status](https://travis-ci.org/dimbleby/ip-rs.svg?branch=master)](https://travis-ci.org/dimbleby/ip-rs)
[![Build status](https://ci.appveyor.com/api/projects/status/ptcqa6x59vy82437/branch/master?svg=true)](https://ci.appveyor.com/project/dimbleby/ip-rs/branch/master)
[![crates.io](http://meritbadge.herokuapp.com/ip)](https://crates.io/crates/ip)

## Documentation ##

API documentation is [here](http://dimbleby.github.io/ip-rs).

## Installation ##

In `Cargo.toml`:

```toml
[dependencies]
ip = "*"
```

And in your crate root:

```rust
extern crate c_types;
```

## Contributing ##

I don't expect to expand the scope of this crate, so it ought to be pretty stable.  Nevertheless, issues and pull requests are welcome.
