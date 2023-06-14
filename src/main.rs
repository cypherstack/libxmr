// This file mostly exists in order to just functionality test via `cargo run`

use monero_serai::{
    // random_scalar,
    // rpc::{HttpRpc, Rpc},
    wallet::{
        // ViewPair, Scanner,
        // address::{AddressError, Network, AddressType, AddressSpec, AddressMeta, MoneroAddress},
        // SpendableOutput,
        seed::{
            Seed, Language
        },
    },
};
// use monero_serai::*;

use rand_core::OsRng; // for generating a seed

fn main() {
    let seed = Seed::new(&mut OsRng, Language::English);
    println!("{:?}", Seed::to_string(&seed));
}
