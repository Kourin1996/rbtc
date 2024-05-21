use std::io::{BufRead, Write};
use std::time::SystemTimeError;
use thiserror::Error;

pub struct BtcMessageEncodingType(pub u32);

pub const BASE_ENCODING: BtcMessageEncodingType = BtcMessageEncodingType(1);

pub const WITNESS_ENCODING: BtcMessageEncodingType = BtcMessageEncodingType(2);

#[derive(Error, Debug)]
pub enum BtcMessageEncodeError {
    #[error("invalid message func={0}, reason={1}")]
    InvalidData(String, String),

    #[error("encoding error func={0}, reason={1}")]
    EncodingError(String, std::io::Error),

    #[error("invalid timestamp error func={0}, reason={1}")]
    InvalidTimestampError(String, SystemTimeError),

    #[error("unknown error func={0}, error={1}")]
    Unknown(String, String),
}

#[derive(Error, Debug)]
pub enum BtcMessageDecodeError {
    #[error("unknown error func={0}, error={1}")]
    Unknown(String, String),
}

pub trait BtcMessage {
    fn encode(
        &self,
        writer: &mut impl Write,
        protocol_version: u32,
        encoding: BtcMessageEncodingType,
    ) -> Result<(), BtcMessageEncodeError>;
    fn decode(
        &self,
        reader: &mut impl BufRead,
        protocol_version: u32,
        encoding: BtcMessageEncodingType,
    ) -> Result<(), BtcMessageDecodeError>;
}
