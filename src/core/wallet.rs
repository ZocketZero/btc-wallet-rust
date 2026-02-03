use bitcoin::{Address, Network};
use secp256k1::{PublicKey, SecretKey};

use crate::{
    core::generate::{generate_keypair_from_hash, generate_keypair_from_text},
    utils::PrintMode,
};

pub struct Wallet {
    private_key_wif: String,
    public_key_str: String,
    address: Address,
}

impl Wallet {
    pub fn gen_wallet(secret_key: SecretKey, pub_key_secp: PublicKey, compressed: bool) -> Self {
        // Construct bitcoin::PrivateKey with specified compressed state
        let private_key = bitcoin::PrivateKey {
            compressed,
            network: Network::Bitcoin,
            inner: secret_key, // Corrected field name
        };
        let private_key_wif = private_key.to_wif();

        // Construct bitcoin::PublicKey with specified compressed state
        let public_key_bitcoin = bitcoin::PublicKey {
            compressed,
            inner: pub_key_secp,
        };
        let public_key_str = public_key_bitcoin.to_string();

        // Convert to a Bitcoin Address for a sharable format
        let address = Address::p2pkh(&public_key_bitcoin, Network::Bitcoin);

        Self {
            private_key_wif,
            public_key_str,
            address,
        }
    }

    pub fn from_hash(hash: &[u8; 32], compressed: bool) -> Self {
        let (secret_key, pub_key_secp) = generate_keypair_from_hash(hash);
        Self::gen_wallet(secret_key, pub_key_secp, compressed)
    }

    pub fn new(text: &str, compressed: bool) -> Self {
        let (secret_key, pub_key_secp) = generate_keypair_from_text(text);
        Self::gen_wallet(secret_key, pub_key_secp, compressed)
    }

    pub fn print(&self, mode: PrintMode, raw: bool) {
        match mode {
            PrintMode::A | PrintMode::All => {
                if raw {
                    println!("{}", self.private_key_wif);
                    println!("{}", self.public_key_str);
                    println!("{}", self.address);
                } else {
                    println!("Private Key (WIF):\n{}\n", self.private_key_wif);
                    println!("Public Key (hex):\n{}\n", self.public_key_str);
                    println!("Bitcoin Address (P2PKH):\n{}\n", self.address);
                }
            }
            PrintMode::S | PrintMode::Secret => {
                if raw {
                    print!("{}", self.private_key_wif);
                } else {
                    println!("Private Key (WIF):\n{}", self.private_key_wif);
                }
            }
            PrintMode::P | PrintMode::Public => {
                if raw {
                    print!("{}", self.address);
                } else {
                    println!("Public Key (hex):\n{}\n", self.public_key_str);
                    println!("Bitcoin Address (P2PKH):\n{}", self.address);
                }
            }
        }
    }
}
