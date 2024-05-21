use crate::protocol::net_address::NetAddress;
use crate::protocol::version::NET_ADDRESS_TIME_VERSION;
use std::io::Write;
use std::net::IpAddr;
use std::time::{SystemTimeError, UNIX_EPOCH};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NetAddressEncodeError {
    #[error("encoding error field={0}, reason={1}")]
    EncodingError(String, std::io::Error),

    #[error("invalid timestamp error field={0}, reason={1}")]
    InvalidTimestampError(String, SystemTimeError),
}

pub fn encode_net_address(
    writer: &mut impl Write,
    net_address: &NetAddress,
    protocol_version: u32,
    includes_timestamp: bool,
) -> Result<(), NetAddressEncodeError> {
    if includes_timestamp && protocol_version > NET_ADDRESS_TIME_VERSION {
        writer
            .write_all(
                &net_address
                    .last_seen_timestamp
                    .duration_since(UNIX_EPOCH)
                    .or_else(|e| {
                        Err(NetAddressEncodeError::InvalidTimestampError(
                            "last_seen_timestamp".to_string(),
                            e,
                        ))
                    })?
                    .as_secs()
                    .to_le_bytes(),
            )
            .or_else(|e| {
                Err(NetAddressEncodeError::EncodingError(
                    "last_seen_timestamp".to_string(),
                    e,
                ))
            })?;
    }

    writer
        .write_all(&net_address.services.to_u64().to_le_bytes())
        .or_else(|e| {
            Err(NetAddressEncodeError::EncodingError(
                "services".to_string(),
                e,
            ))
        })?;

    let ip_bytes = match net_address.ip {
        IpAddr::V4(ip) => ip.octets().to_vec(),
        IpAddr::V6(ip) => ip.octets().to_vec(),
    };

    writer
        .write_all(&ip_bytes)
        .or_else(|e| Err(NetAddressEncodeError::EncodingError("ip".to_string(), e)))?;
    writer
        .write_all(&net_address.port.to_be_bytes())
        .or_else(|e| Err(NetAddressEncodeError::EncodingError("port".to_string(), e)))?;

    Ok(())
}
