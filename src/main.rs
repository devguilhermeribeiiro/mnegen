use bip39::{Language, Mnemonic};
use std::io;

fn main() {
    let mut user_entry = String::new();
    println!("Enter your entropy (16 random bytes in HEX):");
    io::stdin().read_line(&mut user_entry).unwrap();
    user_entry.pop();
    let entropy = hex::decode(&user_entry).unwrap();

    let mut passphrase = String::new();
    println!("Enter your passphrase:");
    io::stdin().read_line(&mut passphrase).unwrap();
    passphrase.pop();

    let mnemonic = Mnemonic::from_entropy_in(Language::English, &entropy).unwrap();
    println!("MNEMONIC:\n{}\n", mnemonic);

    let seed = mnemonic.to_seed(passphrase);
    println!("BIP39 SEED WITH PASSPHRASE:\n{}\n", hex::encode(seed));
}
