#![allow(unused_variables)]
use std::slice;
pub fn raw_pointers () {
    // you can declare raw pointers outside of unsafe rust blocks, but they cannot be dereferenced outside of an unsafe block
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
}

pub fn raw_pointer_arbitrary () {
    let address = 0x012345usize;
    let r = address as *const i32;
}

pub fn deref_raw_pointer () {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}

pub unsafe fn dangerous() {
    println!("Running an unsafe function");
}

pub fn using_split_at_mut () {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}


pub fn my_split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
