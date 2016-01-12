//! A crate that provides the `IpAddr` type, which can represent either an IPv4
//! or an IPv6 address.

use std::fmt;
use std::net::{AddrParseError, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
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

/// Methods to convert between `IpAddr` and `std::net::SocketAddr`
pub trait SocketAddrExt {
    /// Creates a new socket address from the (ip, port) pair.
    fn new(ip: IpAddr, port: u16) -> SocketAddr;
    /// Returns the IP address associated with this socket address.
    fn ip(&self) -> IpAddr;
}

impl SocketAddrExt for SocketAddr {
    fn new(ip: IpAddr, port: u16) -> SocketAddr {
        match ip {
            IpAddr::V4(ipv4_addr) => SocketAddr::V4(SocketAddrV4::new(ipv4_addr, port)),
            IpAddr::V6(ipv6_addr) => SocketAddr::V6(SocketAddrV6::new(ipv6_addr, port, 0, 0)),
        }
    }

    fn ip(&self) -> IpAddr {
        match *self {
            SocketAddr::V4(socket_addr_v4) => IpAddr::V4(*socket_addr_v4.ip()),
            SocketAddr::V6(socket_addr_v6) => IpAddr::V6(*socket_addr_v6.ip()),
        }
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

#[test]
fn to_from_socket_addr() {
    let set_ip = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
    let set_port = 45666;
    let socket_addr = <SocketAddr as SocketAddrExt>::new(set_ip, set_port);
    let get_ip = <SocketAddr as SocketAddrExt>::ip(&socket_addr);
    let get_port = socket_addr.port();
    assert_eq!(set_ip, get_ip);
    assert_eq!(set_port, get_port);
}

