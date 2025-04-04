/*
                        docker build -t mnemonics .
                        docker run -d --name mnemonics-container mnemonics
                        docker logs --follow mnemonics-container
                        sudo docker exec -it mnemonics-container bash
*/
use std::str::FromStr;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead};
use std::io::prelude::*;
use std::path::Path;
use anyhow::Result;
use fs4::fs_std::FileExt;
use std::time::Instant;


use bip39::{Mnemonic, Language};
use bitcoin::{bip32, Address, Network, PublicKey};
use bitcoin::bip32::DerivationPath;

struct Check {
    bulundu : bool,
    cuzdan : Address,
}

impl Check {
    fn new() -> Self {
        Check { bulundu:false, cuzdan:"32iVBEu4dxkUQk9dJbZUiBiQdmypcEyJRf".parse::<Address<_>>().unwrap().require_network(Network::Bitcoin).unwrap() }
    }
}

fn main() {
    let mut file = output_file();
    let mut kontrol = Check::new();
    let all_addresses = load_txt();
    let mut say:u64 = 0;

    loop {
        println!("---------------------------------------------------------------------------------------------");
        say = say + 1;
        let start = Instant::now();
        let started_assigning = start.elapsed();
        let mut rng = bip39::rand::thread_rng();
        let mnemonics = Mnemonic::generate_in_with(&mut rng, Language::English, 12).unwrap();
        println!("Mnemonics = {}",mnemonics);

        let seed: [u8; 64] = mnemonics.to_seed("");

        let secp = bitcoin::secp256k1::Secp256k1::new();
        let master_key = bip32::Xpriv::new_master(Network::Bitcoin, &seed).unwrap();

        let derivation_path = DerivationPath::from_str("m/44'/0'/0'/0/0").expect("Invalid derivation path");
        let derived_xpriv = master_key.derive_priv(&secp, &derivation_path).expect("Failed to derive child key");
        let private_key = derived_xpriv.private_key;

        let key = private_key.public_key(&secp);
        let public_key = PublicKey::new(key);

        let p2pkh_address = Address::p2pkh(&public_key, Network::Bitcoin);
        let add = p2pkh_address;
        println!("Address = {}",add);

        for line in &all_addresses {
            if line.contains(&add) {
                kontrol.cuzdan = add;
                kontrol.bulundu = true;
                break;
            };
        };

        if kontrol.bulundu {
            println!("Bulunan Cuzdan = {}",kontrol.cuzdan);
            file.lock_exclusive().expect("Couldn't lock file.");
            writeln!(file, "{} - {}",mnemonics,kontrol.cuzdan).expect("Couldn't write to `efficient_addresses.txt` file.");
            file.unlock().expect("Couldn't unlock file.");
            break;
        }
        let finished_assigning = start.elapsed();
        println!("{} - Süre = {:?},",say, finished_assigning - started_assigning);
    }
}

fn load_txt() -> Result<Vec<Address>> {
    let path = "adressess/test.txt";
    let mut all_adress: Vec<Address> = Vec::new();

    // Open the file
    if let Ok(file) = File::open(&Path::new(path)) {
        // Read the file line by line using a BufReader
        println!("Reading Started ...");
        for line in io::BufReader::new(file).lines() {
            if let Ok(content) = line {
                all_adress.push(Address::from_str(&content).unwrap().assume_checked());
            }
        }
    }
    println!("Reading Finished ...");
    Ok(all_adress)
}

#[track_caller]
fn output_file() -> File {
    OpenOptions::new()
        .append(true)
        .create(true)
        .read(true)
        .open("adressess/efficient_addresses.txt")
        .expect("Could not create or open `efficient_addresses.txt` file.")
}