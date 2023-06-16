use monero_serai::{
    wallet::{
        seed::{
            Seed, Language
        },
    },
};

use rand_core::OsRng; // for generating a seed
use std::os::raw::{c_char};
use std::ffi::CString;

#[no_mangle] pub extern "C" fn generate_seed() -> *const c_char { // TODO rename fn to be more in line with other libs/impls
    let zeroized_string = &Seed::to_string(&Seed::new(&mut OsRng, Language::English));
    
    // Convert the zeroized string to a normal string
    let rust_string = zeroized_string.as_str();

    // Convert the string to a CString
    let c_string = CString::new(rust_string).expect("Failed to create CString");

    // Convert the CString to a raw pointer
    let raw_ptr = c_string.as_ptr();

    // Clear memory
    std::mem::forget(c_string);

    // Return the raw pointer
    raw_ptr
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
