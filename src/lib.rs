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

#[no_mangle] pub extern "C" fn generate_seed() -> *const c_char { // TODO rename fn to be more in line with other libs/impls
    convert_zeroize_string_to_c_string(&Seed::to_string(&Seed::new(&mut OsRng, Language::English)))
     // TODO add lang as param
}

#[no_mangle] pub extern "C" fn generate_address(mnemonic: *const c_char, account: u32, index: u32) -> *const c_char {
    let seed = Seed::from_string(Zeroizing::new(convert_c_char_ptr_to_string(mnemonic))).unwrap(); // TODO catch empty mnemonic, validate it

    let spend: [u8; 32] = *seed.entropy();
    let spend_scalar = Scalar::from_bytes_mod_order(spend);
    let spend_point: EdwardsPoint = &spend_scalar * &ED25519_BASEPOINT_TABLE;

    let view: [u8; 32] = Keccak256::digest(&spend).into();
    let view_scalar = Scalar::from_bytes_mod_order(view);
    let view_point: EdwardsPoint = &view_scalar * &ED25519_BASEPOINT_TABLE;

    let address: MoneroAddress;
    if (account == 0) && (index == 0) {
        address = MoneroAddress::new(
            AddressMeta::new(Network::Mainnet, AddressType::Standard),
            spend_point,
            view_point,
        );
    } else {
        let view = ViewPair::new(spend_point, Zeroizing::new(view_scalar));
        address = view.address(Network::Mainnet, AddressSpec::Subaddress(SubaddressIndex::new(account, index).unwrap()));
    }
    // TODO network param for Stagenet etc

    let c_string = CString::new(address.to_string()).unwrap();
    let pointer: *const c_char = c_string.as_ptr() as *const c_char;
    std::mem::forget(c_string); // warning: memory leak! must free this memory once done with it
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

fn convert_c_char_ptr_to_string(c_char_ptr: *const c_char) -> String { // TODO return Result<String, Error>
    let c_str: &CStr = unsafe {
        assert!(!c_char_ptr.is_null());

        CStr::from_ptr(c_char_ptr)
    }; // TODO validate that this pointer points to a null-terminated string
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
