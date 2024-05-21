use std::fmt;

#[derive(Debug)]
pub struct ServiceFlag(pub u64);

pub const SF_NODE_NETWORK: ServiceFlag = ServiceFlag(1);
pub const SF_NODE_GET_UTXO: ServiceFlag = ServiceFlag(2);
pub const SF_NODE_BLOOM: ServiceFlag = ServiceFlag(4);
pub const SF_NODE_WITNESS: ServiceFlag = ServiceFlag(8);
pub const SF_NODE_XTHIN: ServiceFlag = ServiceFlag(16);
pub const SF_NODE_BIT5: ServiceFlag = ServiceFlag(32);
pub const SF_NODE_CF: ServiceFlag = ServiceFlag(64);
pub const SF_NODE_2X: ServiceFlag = ServiceFlag(128);
pub const SF_NODE_NETWORK_LIMITED: ServiceFlag = ServiceFlag(1024);

const SF_FLAGS: &'static [ServiceFlag] = &[
    SF_NODE_NETWORK,
    SF_NODE_GET_UTXO,
    SF_NODE_BLOOM,
    SF_NODE_WITNESS,
    SF_NODE_XTHIN,
    SF_NODE_BIT5,
    SF_NODE_CF,
    SF_NODE_2X,
    SF_NODE_NETWORK_LIMITED,
];

const SF_FLAG_NAMES: &'static [&'static str] = &[
    "SFNodeNetwork",
    "SFNodeGetUTXO",
    "SFNodeBloom",
    "SFNodeWitness",
    "SFNodeXthin",
    "SFNodeBit5",
    "SFNodeCF",
    "SFNode2X",
    "SFNodeNetworkLimited",
];

impl ServiceFlag {
    pub fn has_service(&self, flag: ServiceFlag) -> bool {
        self.0 & flag.0 == flag.0
    }

    pub fn add_service(&mut self, flag: ServiceFlag) {
        self.0 |= flag.0;
    }

    pub fn to_u64(&self) -> u64 {
        self.0
    }
}

impl fmt::Display for ServiceFlag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut v = self.0;

        if v == 0 {
            return write!(f, "0x0");
        }

        let mut s: String = "".to_owned();

        for (i, ServiceFlag(f)) in SF_FLAGS.iter().enumerate() {
            if f & v == *f {
                s.push_str(SF_FLAG_NAMES[i]);
                s.push_str("|");
                v -= f;
            }
        }

        let s = if v == 0 {
            s.strip_suffix("|").map(|s| s.to_owned()).unwrap_or(s)
        } else {
            s.push_str(format!("0x{:x}", v).as_str());
            s
        };

        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_flag_format() {
        let tests: Vec<(u64, &str)> = vec![
            (0, "0x0"),
            (SF_NODE_NETWORK.0, "SFNodeNetwork"),
            (SF_NODE_GET_UTXO.0, "SFNodeGetUTXO"),
            (SF_NODE_BLOOM.0, "SFNodeBloom"),
            (SF_NODE_WITNESS.0, "SFNodeWitness"),
            (SF_NODE_XTHIN.0, "SFNodeXthin"),
            (SF_NODE_BIT5.0, "SFNodeBit5"),
            (SF_NODE_CF.0, "SFNodeCF"),
            (SF_NODE_2X.0, "SFNode2X"),
            (SF_NODE_NETWORK_LIMITED.0, "SFNodeNetworkLimited"),
            (0xffffffff, "SFNodeNetwork|SFNodeGetUTXO|SFNodeBloom|SFNodeWitness|SFNodeXthin|SFNodeBit5|SFNodeCF|SFNode2X|SFNodeNetworkLimited|0xfffffb00"),
            (0xfffffb00, "0xfffffb00")
        ];

        for (flag, expected) in tests {
            let sf = ServiceFlag(flag);
            assert_eq!(format!("{}", sf), expected);
        }
    }
}
