use std::io::Write;

pub enum EncodeLiteralType {
    Bool(bool),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    VariableUint(u64),
    String(String),
}

pub fn encode_literal(
    writer: &mut impl Write,
    element: EncodeLiteralType,
) -> Result<(), std::io::Error> {
    match element {
        EncodeLiteralType::Bool(value) => {
            let v: [u8; 1] = if value { [0x01] } else { [0x00] };
            writer.write_all(v.as_ref())?;
        }
        EncodeLiteralType::I32(value) => {
            writer.write_all(&value.to_le_bytes())?;
        }
        EncodeLiteralType::U32(value) => {
            writer.write_all(&value.to_le_bytes())?;
        }
        EncodeLiteralType::I64(value) => {
            writer.write_all(&value.to_le_bytes())?;
        }
        EncodeLiteralType::U64(value) => {
            writer.write_all(&value.to_le_bytes())?;
        }
        EncodeLiteralType::VariableUint(value) => {
            if value < 0xFD {
                writer.write_all(&(value as u8).to_le_bytes())?;
            } else if value <= u16::MAX as u64 {
                writer.write_all(&[0xFD])?;
                writer.write_all(&(value as u16).to_le_bytes())?;
            } else if value <= u32::MAX as u64 {
                writer.write_all(&[0xFE])?;
                writer.write_all(&(value as u32).to_le_bytes())?;
            } else {
                writer.write_all(&[0xFF])?;
                writer.write_all(&value.to_le_bytes())?;
            }
        }
        EncodeLiteralType::String(value) => {
            encode_literal(writer, EncodeLiteralType::VariableUint(value.len() as u64))?;
            writer.write_all(value.as_bytes())?;
        }
    }

    Ok(())
}

pub fn encode_literals(
    writer: &mut impl Write,
    values: Vec<EncodeLiteralType>,
) -> Result<(), std::io::Error> {
    for v in values {
        encode_literal(writer, v)?;
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::encoding::literal::EncodeLiteralType::{
        Bool, String, VariableUint, I32, I64, U32, U64,
    };

    #[test]
    pub fn test_encode_literals() {
        let str256 = "test".repeat(64);

        let tests = vec![
            (Bool(false), vec![0x00]),
            (Bool(true), vec![0x01]),
            (I32(1), vec![0x01, 0x00, 0x00, 0x00]),
            (U32(256), vec![0x00, 0x01, 0x00, 0x00]),
            (
                I64(65536),
                vec![0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00],
            ),
            (
                U64(4294967296),
                vec![0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00],
            ),
            (VariableUint(0), vec![0x00]),
            (VariableUint(0xfc), vec![0xfc]),
            (VariableUint(0xfd), vec![0xfd, 0xfd, 0x00]),
            (VariableUint(0xffff), vec![0xfd, 0xff, 0xff]),
            (VariableUint(0x10000), vec![0xfe, 0x00, 0x00, 0x01, 0x00]),
            (VariableUint(0xffffffff), vec![0xfe, 0xff, 0xff, 0xff, 0xff]),
            (
                VariableUint(0x100000000),
                vec![0xff, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00],
            ),
            (
                VariableUint(0xffffffffffffffff),
                vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff],
            ),
            (String("".to_string()), vec![0x00]),
            (
                String("Test".to_string()),
                vec![0x04, 0x54, 0x65, 0x73, 0x74],
            ),
            (
                String(str256.clone()),
                vec![vec![0xfd, 0x00, 0x01], Vec::from(str256.clone().as_bytes())].concat(),
            ),
        ];

        for (input, expected) in tests {
            let mut writer = Vec::new();
            super::encode_literal(&mut writer, input).unwrap();
            assert_eq!(writer, expected);
        }
    }
}
