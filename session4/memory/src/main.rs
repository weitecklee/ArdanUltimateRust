#![allow(dead_code, clippy::useless_vec)]

/// # Safety
///
/// This function is unsafe because unsafe functions need documentation.
unsafe fn my_fn() {}

fn main() {
    let my_vec = vec![1, 2, 3, 4, 5];
    unsafe {
        let val = my_vec.get_unchecked(20);
        println!("{val}");
    }
}
