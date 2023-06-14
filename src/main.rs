// This file mostly exists in order to just functionality test via `cargo run`

use monero_serai::{
    random_scalar,
    rpc::{HttpRpc, Rpc},
    wallet::{
        ViewPair, Scanner,
        address::{AddressError, Network, AddressType, AddressSpec, AddressMeta, MoneroAddress},
        SpendableOutput,
        seed::{
            Seed, Language
        },
    },
};

fn main() {
    let name = "world";
    println!("Hello, {}!", name);
}
