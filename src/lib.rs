use monero_serai::{
    wallet::{
        seed::{
            Seed, Language
        },
        address::{AddressType, AddressMeta, MoneroAddress, Network},
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

#[no_mangle] pub extern "C" fn generate_seed() -> *const c_char { // TODO rename fn to be more in line with other libs/impls
    convert_zeroize_string_to_c_string(&Seed::to_string(&Seed::new(&mut OsRng, Language::English)))
     // TODO add lang as param
}

#[no_mangle] pub extern "C" fn generate_address(mnemonic: *const c_char) -> *const c_char {
    // let c_str = unsafe {
    //     assert!(!mnemonic.is_null());
    //
    //     CStr::from_ptr(mnemonic)
    // };
    //
    // let r_str = c_str.to_str().unwrap().to_string();

    // let seed = Seed::from_string(Zeroizing::new(r_str)).unwrap(); // TODO catch empty mnemonic, validate it
    let seed = &Seed::new(&mut OsRng, Language::English); // random seed as proof of concept, isolating mnemonic param

    let spend: [u8; 32] = *seed.entropy();
    let spend_scalar = Scalar::from_bytes_mod_order(spend);
    let spend_point: EdwardsPoint = &spend_scalar * &ED25519_BASEPOINT_TABLE;

    let view: [u8; 32] = Keccak256::digest(&spend).into();
    let view_scalar = Scalar::from_bytes_mod_order(view);
    let view_point: EdwardsPoint = &view_scalar * &ED25519_BASEPOINT_TABLE;

    // TODO allow generation of subaddresses via index param (default=0)

    let address = MoneroAddress::new(
        AddressMeta::new(Network::Mainnet, AddressType::Standard),
        spend_point,
        view_point,
    );

    let c_string = CString::new(address.to_string()).unwrap();
    let pointer = c_string.as_ptr() as *const c_char;
    std::mem::forget(c_string);
    pointer
}

fn convert_zeroize_string_to_c_string(zeroized_string: &Zeroizing<String>) -> *const c_char {
    // Convert the zeroized string to a normal string
    let rust_string = zeroized_string.as_str();

    // Convert the string to a CString
    let c_string = CString::new(rust_string).expect("Failed to create CString");

    // Convert the CString to a raw pointer
    let raw_ptr = c_string.into_raw();

    let const_ptr = raw_ptr as *const c_char;
    // mem::forget(const_ptr);

    // Return the raw pointer
    const_ptr
}

fn convert_c_char_ptr_to_str(c_char: *const c_char) -> String { // TODO return Result<String, Error>
    let c_str: &CStr = unsafe { CStr::from_ptr(c_char) }; // TODO validate that this pointer is valid and points to a null-terminated string
    let str_slice: &str = c_str.to_str().unwrap(); // TODO validate that c_str is valid UTF-8
    str_slice.to_string()
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
