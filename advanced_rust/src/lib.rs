#![allow(unused_variables)]
pub mod unsafe_rust;
pub mod advanced_traits;
pub mod advanced_types;
pub mod advanced_functions;
pub mod macros;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
