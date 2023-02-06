use std::slice;
fn unsafe_raw_pointer() {
    let mut num = 5;
    let r1 = &num as *const i32; // immutable raw pointer
    let r2 = &mut num as *mut i32; // mutable raw pointer
    
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    let address = 0x012345usize; // It is an invalid address
    let r3 = address as *const i32; // immutable raw pointer which is pointer to invalid address.
    unsafe {
        println!("r3 is: {}", *r3); // This will fail as the address is invalid
    }
}
fn unsafe_function_check() {
    /* 
        Unsafe function can be called from unsafe function or unsafe block
    */
    unsafe fn dangerous() {}
    unsafe {
        dangerous();
    }
}
fn safe_abstraction_on_unsafe_code_v1() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}
safe_abstraction_on_unsafe_code_v2(slice: &mut[i32], mid: usize)  -> (slice: &mut[i32], slice: &mut[i32]){
    let len = slice.len();
    
    assert!(mid <= len);
    (&mut slice[..mid], &mut slice[mid..])

}
safe_abstraction_on_unsafe_code_v3(slice: &mut[i32], mid: usize) -> (&mut[i32], &mut[i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);
    unsafe {
        (   slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
        
    }
}
extern "C" {
    fn abs(input: i32) -> i32;
}
fn calling_unsafe_extern_func() {
    unsafe {
        println!("Calling unsafe Function from External Interface: {}", abs(-3));
    }
}
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust Function from C!!");
}
fn main() {
    // unsafe_raw_pointer();
    // unsafe_function_check();
    // safe_abstraction_on_unsafe_code_v1();
    // safe_abstraction_on_unsafe_code_v2();
    // safe_abstraction_on_unsafe_code_v3();
    // calling_unsafe_extern_func();
}

// Trait is unsafe if any of the method is unsafe
// Static keyword based operation can be also considered as unsafe as multiple threads can access and change the value
// In Rust static keyword based variable is considered as global variable.
