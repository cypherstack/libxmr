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
    constants::ED25519_BASEPOINT_POINT, // for generating an address TODO remove after generating address from Seed vs basepoint
    edwards::{EdwardsPoint},
    // scalar::Scalar,
};

use sha3::{Digest, Keccak256}; // for generating the view key

use monero_generators::{hash_to_point}; // for [u8; 32] -> EdwardsPoint conversion

fn main() {
    // generate seed & return as mnemonic
    let seed = &Seed::new(&mut OsRng, Language::English); // TODO input lang as param
    println!("{:?}", Seed::to_string(seed));

    // generate address
    let spend: [u8; 32] = *seed.entropy();
    let spend_point: EdwardsPoint = hash_to_point(spend);
    let view: [u8; 32] = Keccak256::digest(&spend).into();
    let view_point: EdwardsPoint = hash_to_point(view);
    println!("{:?}", 
            MoneroAddress::new(
              AddressMeta::new(Network::Mainnet, AddressType::Standard),
              spend_point,
              view_point,
            ),
        );
}
