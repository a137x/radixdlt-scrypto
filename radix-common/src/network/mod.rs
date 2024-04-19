use radix_rust::rust::str::FromStr;
use radix_rust::rust::string::String;
use sbor::*;

/// Network Definition is intended to be the actual definition of a network
#[derive(Debug, Clone, Sbor, PartialEq, Eq)]
pub struct NetworkDefinition {
    pub id: u8,
    pub logical_name: String,
    pub hrp_suffix: String,
}

// NOTE: Most Network Definitions live in the node codebase
// Some are duplicated here so that they can be easily used by scrypto and resim
impl NetworkDefinition {
    pub fn simulator() -> NetworkDefinition {
        NetworkDefinition {
            id: 242,
            logical_name: String::from("simulator"),
            hrp_suffix: String::from("sim"),
        }
    }

    /// The network definition for Alphanet
    pub fn adapanet() -> NetworkDefinition {
        NetworkDefinition {
            id: 0x0a,
            logical_name: String::from("adapanet"),
            hrp_suffix: String::from("tdx_a_"),
        }
    }

    /// The network definition for Betanet
    pub fn nebunet() -> NetworkDefinition {
        NetworkDefinition {
            id: 0x0b,
            logical_name: String::from("nebunet"),
            hrp_suffix: String::from("tdx_b_"),
        }
    }

    /// The network definition for RCnet v1
    pub fn kisharnet() -> NetworkDefinition {
        NetworkDefinition {
            id: 0x0c,
            logical_name: String::from("kisharnet"),
            hrp_suffix: String::from("tdx_c_"),
        }
    }

    /// The network definition for RCnet v2
    pub fn ansharnet() -> NetworkDefinition {
        NetworkDefinition {
            id: 0x0d,
            logical_name: String::from("ansharnet"),
            hrp_suffix: String::from("tdx_d_"),
        }
    }

    /// The network definition for RCnet v3
    pub fn zabanet() -> NetworkDefinition {
        NetworkDefinition {
            id: 0x0e,
            logical_name: String::from("zabanet"),
            hrp_suffix: String::from("tdx_e_"),
        }
    }

    pub fn stokenet() -> NetworkDefinition {
        NetworkDefinition {
            id: 2,
            logical_name: String::from("stokenet"),
            hrp_suffix: String::from("tdx_2_"),
        }
    }

    pub fn mainnet() -> NetworkDefinition {
        NetworkDefinition {
            id: 1,
            logical_name: String::from("mainnet"),
            hrp_suffix: String::from("rdx"),
        }
    }
}

impl FromStr for NetworkDefinition {
    type Err = ParseNetworkError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "simulator" => Ok(NetworkDefinition::simulator()),
            "adapanet" => Ok(NetworkDefinition::adapanet()),
            "nebunet" => Ok(NetworkDefinition::nebunet()),
            "kisharnet" => Ok(NetworkDefinition::kisharnet()),
            "ansharnet" => Ok(NetworkDefinition::ansharnet()),
            "zabanet" => Ok(NetworkDefinition::zabanet()),
            "stokenet" => Ok(NetworkDefinition::stokenet()),
            "mainnet" => Ok(NetworkDefinition::mainnet()),
            _ => Err(ParseNetworkError::InvalidNetworkString),
        }
    }
}

#[derive(Debug)]
pub enum ParseNetworkError {
    InvalidNetworkString,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn network_from_string_fail() {
        assert!(matches!(
            NetworkDefinition::from_str("non_existing_network").unwrap_err(),
            ParseNetworkError::InvalidNetworkString
        ));
    }

    #[test]
    fn network_ids() {
        let array = [
            ("mainnet", 1),
            ("Simulator", 242),
            ("Adapanet", 10),
            ("NEBUNET", 11),
            ("Kisharnet", 12),
            ("ansharnet", 13),
            ("zabanet", 14),
        ];

        for (name, id) in array {
            assert_eq!(NetworkDefinition::from_str(name).unwrap().id, id)
        }
    }
}
