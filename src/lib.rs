//!
//! This module provides the `unwrap_fmt!` macro for basically
//! unwrapping either `Option` or `Result` with a formatted string.
//!

#![feature(never_type)]

mod fmt_expect;
pub use fmt_expect::FmtExpect;

/// like `expect` but formatted string. works for both `Option` and `Result`
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
