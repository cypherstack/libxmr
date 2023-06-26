// This file mostly exists in order to just test functionality via `cargo run`

use monero_serai::{
    // random_scalar,
    // rpc::{HttpRpc, Rpc},
    wallet::{
        // ViewPair, Scanner,
        // address::{AddressError, Network, AddressType, AddressSpec, AddressMeta, MoneroAddress},
        // SpendableOutput,
        seed::{Seed, Language},
        address::{AddressType, AddressMeta, MoneroAddress, Network},
    },
};
// use monero_serai::*;

use rand_core::OsRng; // for generating a seed

use curve25519_dalek::{
    edwards::EdwardsPoint,
    scalar::Scalar,
};

use sha3::{Digest, Keccak256}; // for generating the view key

use monero_generators::{hash_to_point}; // for [u8; 32] -> EdwardsPoint conversion

use zeroize::Zeroizing;

fn main() {
    println!("Running generation check...");
    // generate random seed & print as mnemonic
    let seed = &Seed::new(&mut OsRng, Language::English); // TODO input lang as param
    println!("Seed (mnemonic): {:?}", Seed::to_string(seed));

    // generate address from seed generated above
    let spend: [u8; 32] = *seed.entropy();
    println!("Private spend key: {:?}", hex::encode(spend));
    let spend_point: EdwardsPoint = hash_to_point(spend);
    let view: [u8; 32] = Keccak256::digest(&spend).into();
    println!("Private view key: {:?}", hex::encode(view));
    let view_point: EdwardsPoint = hash_to_point(view);
    // println!("{:?}", 
    //         MoneroAddress::new(
    //           AddressMeta::new(Network::Mainnet, AddressType::Standard),
    //           spend_point,
    //           view_point,
    //         ),
    //     );

    println!("\nRunning checks on test vector...");
    // run same tests on test vector; see https://xmrtests.llcoins.net/addresstests.html
    // seed (mnemonic): hemlock jubilee eden hacksaw boil superior inroads epoxy exhale orders cavernous second brunt saved richly lower upgrade hitched launching deepest mostly playful layout lower eden
    // seed (hex): 29adefc8f67515b4b4bf48031780ab9d071d24f8a674b879ce7f245c37523807
    // private spend: 29adefc8f67515b4b4bf48031780ab9d071d24f8a674b879ce7f245c37523807
    // private view: 3bc0b202cde92fe5719c3cc0a16aa94f88a5d19f8c515d4e35fae361f6f2120e
    let mnemonic = "hemlock jubilee eden hacksaw boil superior inroads epoxy exhale orders cavernous second brunt saved richly lower upgrade hitched launching deepest mostly playful layout lower eden";
    let seed = Seed::from_string(Zeroizing::new(mnemonic.to_string())).unwrap();
    println!("Seed (mnemonic): {:?}", Seed::to_string(&seed));

    let spend: [u8; 32] = *seed.entropy();
    println!("Private spend key: {:?}", hex::encode(spend));
    let spend_point: EdwardsPoint = hash_to_point(spend);
    let view: [u8; 32] = Keccak256::digest(&spend).into();
    let view_scalar = Scalar::from_bytes_mod_order(view);
    println!("Private view key: {:?}", hex::encode(view_scalar.to_bytes()));
    let view_point: EdwardsPoint = hash_to_point(view_scalar.to_bytes()); // TODO should view or view_scalar be used here? we'll find out through the process of address encoding
}
