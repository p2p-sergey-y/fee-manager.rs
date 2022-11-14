use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt};

#[derive(Serialize, Default)]
pub struct Config {
    pub db: HashMap<BLSPubkey, Eth1Address>,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(try_from = "&str")]
pub struct BLSPubkey(String);

impl TryFrom<&str> for BLSPubkey {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() != 98 {
            Err("The length of pubkey should be 98")
        } else if value.get(0..2).unwrap() != "0x" {
            Err("Pubkey should start from '0x'")
        } else {
            Ok(BLSPubkey(value.to_lowercase()))
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(try_from = "&str")]
pub struct Eth1Address(String);

impl fmt::Display for Eth1Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for Eth1Address {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() != 40 {
            Err("The length of pubkey should be 40")
        } else if value.get(0..2).unwrap() != "0x" {
            Err("Pubkey should start from '0x'")
        } else {
            Ok(Eth1Address(value.to_lowercase()))
        }
    }
}

impl TryFrom<String> for Eth1Address {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() != 40 {
            Err("The length of pubkey should be 40")
        } else if value.get(0..2).unwrap() != "0x" {
            Err("Pubkey should start from '0x'")
        } else {
            Ok(Eth1Address(value.to_lowercase()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bls_convert() {
        assert_eq!(
            BLSPubkey::try_from("123"),
            Err("The length of pubkey should be 98")
        );
        assert_eq!(
            BLSPubkey::try_from(""),
            Err("The length of pubkey should be 98")
        );
        assert_eq!(BLSPubkey::try_from("0xaff3071a4bff828acb8e0c6e0c73aa481162da7b8735fd798c3237bc643ae1aa325d6281abe61ffedaf2eb51b29eaf271"), Err("The length of pubkey should be 98"));
        assert_eq!(BLSPubkey::try_from("1xaff3071a4bff828acb8e0c6e0c73aa481162da7b8735fd798c3237bc643ae1aa325d6281abe61ffedaf2eb51b29eaf27"), Err("Pubkey should start from '0x'"));
        assert_eq!(BLSPubkey::try_from("0baff3071a4bff828acb8e0c6e0c73aa481162da7b8735fd798c3237bc643ae1aa325d6281abe61ffedaf2eb51b29eaf27"), Err("Pubkey should start from '0x'"));
        assert!(BLSPubkey::try_from("0xaff3071a4bff828acb8e0c6e0c73aa481162da7b8735fd798c3237bc643ae1aa325d6281abe61ffedaf2eb51b29eaf27").is_ok());
    }
}
