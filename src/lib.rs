//!
//! This module provides the `unwrap_fmt!` macro for basically
//! unwrapping either `Option` or `Result` with a formatted string.
//!

// #![feature]

/// like `expect` but formatted string. works for both `Option` and `Result`
#[macro_export]
macro_rules! unwrap_fmt {
    ($expr:expr, $($rest:tt),*) => {
        FmtExpect::expect_fmt($expr, || panic!($($rest),*))
    };
}

/// Use the `unwrap_that_mf!` macro instead.
///
/// The only reason this exists is because `Option` and `Result`
/// have a different number of arguments for `unwrap_or_else` so
/// it's not possible to use the same macro for both.
pub trait FmtExpect<T> {
    fn expect_fmt(self, panic: impl FnOnce()) -> T;
}

impl<T> FmtExpect<T> for Option<T> {
    fn expect_fmt(self, panic: impl FnOnce()) -> T {
        self.unwrap_or_else(|| {
            panic();
            unreachable!()
        })
    }
}

impl<T, E> FmtExpect<T> for Result<T, E> {
    fn expect_fmt(self, panic: impl FnOnce()) -> T {
        // ignore the damn argument!!!
        self.unwrap_or_else(|_| {
            panic();
            unreachable!()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unwrap_that_some() {
        let opt = Some(1);
        let n = unwrap_fmt!(opt, "failed, beep boop.");
        assert_eq!(n, 1);
    }

    #[test]
    #[should_panic(expected = "expected 1, got None")]
    fn unwrap_that_none() {
        let opt: Option<i32> = None;
        unwrap_fmt!(opt, "expected {}, got {:?}", 1, opt);
    }

    #[test]
    fn unwrap_that_ok() {
        let res: Result<i32, ()> = Ok(1);
        let n = unwrap_fmt!(res, "failed, beep boop.");
        assert_eq!(n, 1);
    }

    #[test]
    #[should_panic(expected = "expected 1, got Err(())")]
    fn unwrap_that_err() {
        let res: Result<i32, ()> = Err(());
        unwrap_fmt!(res, "expected {}, got {:?}", 1, res);
    }
}
