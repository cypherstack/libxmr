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

use curve25519_dalek::constants::ED25519_BASEPOINT_POINT; // for generating an address TODO remove after generating address from Seed vs basepoint

fn main() {
    // generate seed & return as mnemonic
    let seed = &Seed::new(&mut OsRng, Language::English); // TODO input lang as param
    println!("{:?}", Seed::to_string(seed));

    // generate address
    println!("{:?}", 
            MoneroAddress::new(
              AddressMeta::new(Network::Mainnet, AddressType::Standard),
              ED25519_BASEPOINT_POINT, // TODO spend key from Seed above
              ED25519_BASEPOINT_POINT, // TODO view key from Seed above
            ),
        );
}
