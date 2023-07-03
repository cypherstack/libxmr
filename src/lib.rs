use monero_serai::{
    wallet::{
        seed::{
            Seed, Language
        },
        address::{AddressType, AddressMeta, AddressSpec, MoneroAddress, Network, SubaddressIndex},
        ViewPair,
    },
};

use rand_core::OsRng; // for generating a seed
use zeroize::{Zeroizing};
use std::os::raw::{c_char};
use std::ffi::CString;
use std::ffi::CStr;

use curve25519_dalek::{
    edwards::EdwardsPoint,
    scalar::Scalar,
    constants::ED25519_BASEPOINT_TABLE,
};

use sha3::{Digest, Keccak256}; // for generating the view key

#[no_mangle] pub extern "C" fn generate_mnemonic(language: u8) -> *const c_char {
    // Make language int to Language struct
    let _language: Language = match language{
        0=>Language::German,
        1=>Language::English,
        2=>Language::Spanish,
        3=>Language::French,
        4=>Language::Italian,
        5=>Language::Dutch,
        6=>Language::Portuguese,
        7=>Language::Russian,
        8=>Language::Chinese,
        9=>Language::Japanese,
        10=>Language::Esperanto,
        11=>Language::Lojban,
        12=>Language::EnglishOld,
        _=>Language::English,
    };

    // Convert/cast and return
    let ptr: *const c_char = convert_zeroize_string_to_c_char_ptr(&Seed::to_string(&Seed::new(&mut OsRng, _language)));
    ptr
}

#[no_mangle] pub extern "C" fn generate_address(mnemonic: *const c_char, network: u8, account: u32, index: u32) -> *const c_char {
    let seed: Seed = Seed::from_string(Zeroizing::new(convert_c_char_ptr_to_string(mnemonic))).unwrap(); // TODO validate mnemonic (catch empty etc)

    let _network: Network = match network{
        0=>Network::Mainnet,
        1=>Network::Testnet,
        2=>Network::Stagenet,
        _=>Network::Mainnet,
        // etc
    };

    // Calculate spend key and point
    let spend: [u8; 32] = *seed.entropy();
    let spend_scalar: Scalar = Scalar::from_bytes_mod_order(spend);
    let spend_point: EdwardsPoint = &spend_scalar * &ED25519_BASEPOINT_TABLE;

    // Calculate view key and point
    let view: [u8; 32] = Keccak256::digest(&spend).into();
    let view_scalar: Scalar = Scalar::from_bytes_mod_order(view);
    let view_point: EdwardsPoint = &view_scalar * &ED25519_BASEPOINT_TABLE;

    let address: MoneroAddress;
    if (account == 0) && (index == 0) {
        // Public wallet address
        address = MoneroAddress::new(
            AddressMeta::new(_network, AddressType::Standard),
            spend_point,
            view_point,
        );
    } else {
        // Public wallet subaddress at (account, index)
        let view: ViewPair = ViewPair::new(spend_point, Zeroizing::new(view_scalar));
        address = view.address(_network, AddressSpec::Subaddress(SubaddressIndex::new(account, index).unwrap()));
    }

    // Convert/cast
    let c_string = CString::new(address.to_string()).unwrap(); // TODO validate address
    let ptr: *const c_char = c_string.as_ptr() as *const c_char;

    // Do not clean memory; must be freed by Dart wrapper
    std::mem::forget(c_string); // warning: memory leak! must free this memory once done with it

    // Return
    ptr
}

fn convert_zeroize_string_to_c_char_ptr(zeroized_string: &Zeroizing<String>) -> *const c_char {
    // Convert the zeroized string to a normal string
    let rust_string = zeroized_string.as_str();

    // Convert the string to a CString
    let c_string = CString::new(rust_string).expect("Failed to create CString");

    // Convert the CString to a raw pointer
    let raw_ptr = c_string.into_raw();

    // Cast
    let ptr = raw_ptr as *const c_char;

    // Save in memory; must be freed by Dart wrapper
    // std::mem::forget(raw_ptr); // warning: memory leak! must free this memory once done with it
    // TODO: should this be uncommented?

    // Return the raw pointer
    ptr
}

fn convert_c_char_ptr_to_string(c_char_ptr: *const c_char) -> String { // TODO return Result<String, Error>
    // Make sure c_char_ptr isn't null
    let c_str: &CStr = unsafe {
        assert!(!c_char_ptr.is_null());

        // Convert from pointer to c string
        CStr::from_ptr(c_char_ptr)
    }; // TODO validate that this pointer points to a null-terminated string

    // Convert and return
    let r_string: String = c_str.to_str().unwrap().to_string(); // to_owned()? TODO validate that c_str is valid
    r_string
}

// TODO remove this example fn
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO update tests to use real fns
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
