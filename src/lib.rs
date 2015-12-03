//! A crate that provides the `IpAddr` type, which can represent either an IPv4
//! or an IPv6 address.

use std::fmt;
use std::net::{AddrParseError, Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

/// An IP address, either an IPv4 or IPv6 address.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, PartialOrd, Ord)]
pub enum IpAddr {
    /// Representation of an IPv4 address.
    V4(Ipv4Addr),
    /// Representation of an IPv6 address.
    V6(Ipv6Addr),
}

impl fmt::Display for IpAddr {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IpAddr::V4(ref a) => a.fmt(fmt),
            IpAddr::V6(ref a) => a.fmt(fmt),
        }
    }
}

impl FromStr for IpAddr {
    type Err = AddrParseError;

    fn from_str(s: &str) -> Result<IpAddr, AddrParseError> {
        Ipv4Addr::from_str(s).map(|v4| IpAddr::V4(v4))
            .or_else(|_| {
                Ipv6Addr::from_str(s).map(|v6| IpAddr::V6(v6))
            })
    }
}

#[test]
fn parse_ipv4() {
    let parsed = IpAddr::from_str("192.168.0.1").unwrap();
    let ipv4 = Ipv4Addr::from_str("192.168.0.1").unwrap();
    assert_eq!(IpAddr::V4(ipv4), parsed);
}

#[test]
fn parse_ipv6() {
    let parsed = IpAddr::from_str("2001:0db8::0001").unwrap();
    let ipv6 = Ipv6Addr::from_str("2001:0db8::0001").unwrap();
    assert_eq!(IpAddr::V6(ipv6), parsed);
}

#[test]
fn parse_fails() {
    let parsed = IpAddr::from_str("nonsense");
    assert!(parsed.is_err());
}
