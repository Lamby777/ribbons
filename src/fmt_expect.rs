/// Use the `unwrap_fmt!` macro instead.
///
/// The only reason this exists is because `Option` and `Result`
/// have a different number of arguments for `unwrap_or_else` so
/// it's not possible to use the same macro for both.
pub trait FmtExpect<T> {
    fn expect_fmt(self, panic: impl FnOnce() -> !) -> T;
}

impl<T> FmtExpect<T> for Option<T> {
    fn expect_fmt(self, panic: impl FnOnce() -> !) -> T {
        self.unwrap_or_else(|| panic())
    }
}

impl<T, E> FmtExpect<T> for Result<T, E> {
    fn expect_fmt(self, panic: impl FnOnce() -> !) -> T {
        // ignore the damn argument!!!
        self.unwrap_or_else(|_| panic())
    }
}
