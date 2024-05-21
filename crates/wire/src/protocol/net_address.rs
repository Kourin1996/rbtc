use crate::protocol::flags::ServiceFlag;
use std::net::IpAddr;
use std::time::SystemTime;

#[derive(Debug)]
pub struct NetAddress {
    pub last_seen_timestamp: SystemTime,

    pub services: ServiceFlag,

    pub ip: IpAddr,

    pub port: u16,
}

impl NetAddress {
    pub fn new() -> NetAddress {
        NetAddress {
            last_seen_timestamp: SystemTime::now(),
            services: ServiceFlag(0),
            ip: IpAddr::V4(std::net::Ipv4Addr::new(0, 0, 0, 0)),
            port: 0,
        }
    }
}
