//! There are several problems with this library.
//!
//! Do NOT fix them yet!
//!
//! We will incrementally build a CI pipeline to discover
//! how CI can help us make sure our Rust code is squeaky clean.

pub fn badly_formatted        (
) {
    println!("This code is badly formatted.");
println!("Don't change it until CI fails because of it."); }

pub fn clippy_doesnt_like_this() {
    if i32::MAX < i32::MIN {
        println!("This is ridiculous.");
    }
}

pub fn dangerous_cast(n: i64) -> i32 {
    // In my opinion, the `as` keyword should be used sparingly.
    // This conversion is lossy and may not do what you expect.
    // It is better to replace it with `.try_into()` and return
    // a `Result<i64, _>` instead.
    //
    // But as always, let's get CI to complain about it before we fix it.
    n as i32
}

pub fn add(left: usize, right: usize) -> usize {
    // I don't know how to add two numbers...
    left - right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(5, 3);
        assert_eq!(result, 8);
    }
}
