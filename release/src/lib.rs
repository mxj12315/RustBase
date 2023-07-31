//! # My Crate  crate的整体说明
//! //! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.
pub mod Art;
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5; 
/// let answer = release::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}