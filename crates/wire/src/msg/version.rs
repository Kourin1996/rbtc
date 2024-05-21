use crate::encoding::literal::{encode_literals, EncodeLiteralType};
use crate::encoding::net_address::{encode_net_address, NetAddressEncodeError};
use crate::protocol::flags::ServiceFlag;
use crate::protocol::msg::{
    BtcMessage, BtcMessageDecodeError, BtcMessageEncodeError, BtcMessageEncodingType,
};
use crate::protocol::net_address::NetAddress;
use crate::protocol::version::BIP_0037_VERSION;
use std::io::{BufRead, Write};
use std::time::SystemTime;

const MAX_USER_AGENT_LEN: usize = 256;

#[derive(Debug)]
pub struct MsgVersion {
    pub protocol_version: i32,

    pub services: ServiceFlag,

    pub timestamp: SystemTime,

    pub addr_you: NetAddress,

    pub addr_me: NetAddress,

    pub nonce: u64,

    pub user_agent: String,

    pub last_seen_block: i32,

    pub disable_relay_tx: bool,
}

impl MsgVersion {
    pub fn new() -> MsgVersion {
        MsgVersion {
            protocol_version: 70015,
            services: ServiceFlag(0),
            timestamp: SystemTime::now(),
            addr_you: NetAddress::new(),
            addr_me: NetAddress::new(),
            nonce: 0,
            user_agent: "".to_string(),
            last_seen_block: 0,
            disable_relay_tx: false,
        }
    }

    pub fn has_service(&self, flag: ServiceFlag) -> bool {
        self.services.has_service(flag)
    }

    pub fn add_service(&mut self, flag: ServiceFlag) {
        self.services.add_service(flag);
    }

    pub fn validate_user_agent(user_agent: &str) -> bool {
        user_agent.len() <= MAX_USER_AGENT_LEN
    }
}

impl BtcMessage for MsgVersion {
    fn encode(
        &self,
        writer: &mut impl Write,
        protocol_version: u32,
        encoding: BtcMessageEncodingType,
    ) -> Result<(), BtcMessageEncodeError> {
        if !MsgVersion::validate_user_agent(&self.user_agent) {
            return Err(BtcMessageEncodeError::InvalidData(
                "MsgVersion".to_string(),
                format!(
                    "user agent is too long: len={}, max={}",
                    self.user_agent.len(),
                    MAX_USER_AGENT_LEN
                ),
            ));
        }

        encode_literals(
            writer,
            vec![
                EncodeLiteralType::I32(self.protocol_version),
                EncodeLiteralType::U64(self.services.0),
                EncodeLiteralType::U64(
                    self.timestamp
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .or_else(|e| {
                            Err(BtcMessageEncodeError::InvalidTimestampError(
                                "MsgVersion".to_string(),
                                e,
                            ))
                        })?
                        .as_secs(),
                ),
            ],
        )
        .or_else(|e| {
            Err(BtcMessageEncodeError::EncodingError(
                "MsgVersion".to_string(),
                e,
            ))
        })?;

        encode_net_address(writer, &self.addr_you, protocol_version, true).or_else(
            |e| match e {
                NetAddressEncodeError::InvalidTimestampError(_, e) => Err(
                    BtcMessageEncodeError::InvalidTimestampError("MsgVersion".to_string(), e),
                ),
                NetAddressEncodeError::EncodingError(_, e) => Err(
                    BtcMessageEncodeError::EncodingError("MsgVersion".to_string(), e),
                ),
            },
        )?;

        encode_net_address(writer, &self.addr_me, protocol_version, true).or_else(|e| match e {
            NetAddressEncodeError::InvalidTimestampError(_, e) => Err(
                BtcMessageEncodeError::InvalidTimestampError("MsgVersion".to_string(), e),
            ),
            NetAddressEncodeError::EncodingError(_, e) => Err(
                BtcMessageEncodeError::EncodingError("MsgVersion".to_string(), e),
            ),
        })?;

        encode_literals(
            writer,
            vec![
                EncodeLiteralType::U64(self.nonce),
                EncodeLiteralType::String(self.user_agent.clone()),
                EncodeLiteralType::I32(self.last_seen_block),
            ],
        )
        .or_else(|e| {
            Err(BtcMessageEncodeError::EncodingError(
                "MsgVersion".to_string(),
                e,
            ))
        })?;

        if protocol_version >= BIP_0037_VERSION {
            encode_literals(
                writer,
                vec![EncodeLiteralType::Bool(!self.disable_relay_tx)],
            )
            .or_else(|e| {
                Err(BtcMessageEncodeError::EncodingError(
                    "MsgVersion".to_string(),
                    e,
                ))
            })?;
        }

        Ok(())
    }

    fn decode(
        &self,
        reader: &mut impl BufRead,
        protocol_version: u32,
        encoding: BtcMessageEncodingType,
    ) -> Result<(), BtcMessageDecodeError> {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_encode() {
        let msg = MsgVersion::new();
        let mut buffer = Vec::new();

        let protocol_version = 70015;

        msg.encode(&mut buffer, protocol_version, BtcMessageEncodingType(1))
            .unwrap();

        assert_eq!(buffer, vec![0x7f, 0x11, 0x01, 0x00]);
    }
}
