use monero_serai::{
    wallet::{
        seed::{
            Seed, Language
        },
    },
};

use rand_core::OsRng; // for generating a seed

#[no_mangle] pub extern fn generate_seed() { // TODO rename fn to be more in line with other libs/impls
    Seed::to_string(&Seed::new(&mut OsRng, Language::English)); // TODO add lang as param
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
