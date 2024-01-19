//!
//! This module provides the `unwrap_fmt!` macro for basically
//! unwrapping either `Option` or `Result` with a formatted string.
//!

#![feature(never_type)]

mod fmt_expect;
pub use fmt_expect::FmtExpect;

/// Like `expect` but formatted string.
/// Works for both `Option` and `Result`
///
/// # Examples
///
/// Here's one where it panics. Oh no!
///
/// ```should_panic
/// use ribbons::unwrap_fmt;
/// let opt: Option<()> = None;
///
/// // "expected 1, got None"
/// unwrap_fmt!(opt, "expected 1, got {:?}", opt);
/// ```
///
/// Here's one where it doesn't. Real mature.
///
/// ```
/// use ribbons::unwrap_fmt;
/// let msg = "skill issue";
/// let opt: Option<i32> = Some(69420);
///
/// let funny_number = unwrap_fmt!(
///     opt,
///     "Your code contains a severe {}.",
///     msg
/// );
///
/// assert_eq!(funny_number, 69420);
/// ```
#[macro_export]
macro_rules! unwrap_fmt {
    ($expr:expr, $($rest:tt),*) => {
        $crate::FmtExpect::expect_fmt($expr, || panic!($($rest),*))
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unwrap_some() {
        let opt = Some(1);
        let n = unwrap_fmt!(opt, "failed, beep boop.");
        assert_eq!(n, 1);
    }

    #[test]
    #[should_panic(expected = "expected 1, got None")]
    fn unwrap_none() {
        let opt: Option<i32> = None;
        unwrap_fmt!(opt, "expected {}, got {:?}", 1, opt);
    }

    #[test]
    fn unwrap_ok() {
        let res: Result<i32, ()> = Ok(1);
        let n = unwrap_fmt!(res, "failed, beep boop.");
        assert_eq!(n, 1);
    }

    #[test]
    #[should_panic(expected = "expected 1, got Err(())")]
    fn unwrap_err() {
        let res: Result<i32, ()> = Err(());
        unwrap_fmt!(res, "expected {}, got {:?}", 1, res);
    }
}
