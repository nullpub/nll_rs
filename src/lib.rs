/// Composes any number of functions
///
/// # Example
/// ```
/// # #[macro_use] extern crate nll_rs;
/// # fn main() {
/// use crate::nll_rs::*;
///
/// let composed_fn = compose!(|x| x + 1, |x| x * 2, |x: i32| x.pow(2));
///
/// assert_eq!(composed_fn(1), 16);
///
/// println!("{}", composed_fn(1));
/// # }
/// ```
#[macro_export]
macro_rules! compose {
    ( $last:expr ) => { $last };
    ( $head:expr, $($tail:expr), +) => { |x| compose!($($tail),+)($head(x)) };
}
