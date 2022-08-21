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
// uses C's absolute value function
extern "C" {
    pub fn abs(input: i32) -> i32;
}

// the inverse of using function from an ABI, allowing a rust function to be called by other languages
#[no_mangle]// needed for extern functions that are going to called in other languages
pub extern "C" fn call_from_c() {// unsafe not needed here
    println!("Just called a Rust function from C!");
}

// traits can be unsafe if any of their methods have an invariant the compiler can't identify
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}


