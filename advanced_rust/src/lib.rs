#![allow(unused_variables)]
pub fn raw_pointers () {
    // you can declare raw pointers outside of unsafe rust blocks, but they cannot be dereferenced outside of an unsafe block
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
