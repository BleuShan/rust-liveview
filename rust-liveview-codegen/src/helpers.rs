use proc_macro2::Span;
use proc_macro_error::*;
pub(crate) use rust_liveview_common::*;

pub(crate) trait DarlingResultExt {
    type Ok;

    fn unwrap_or_abort(self) -> Self::Ok;
    fn expect_or_abort<S>(self, message: S) -> Self::Ok
    where
        S: AsRef<str>;
}

impl<T> DarlingResultExt for Result<T, darling::Error> {
    type Ok = T;
    fn unwrap_or_abort(self) -> T {
        match self {
            Ok(v) => v,
            Err(e) => {
                e.emit();
                abort!(Span::call_site(), "An error occured.")
            }
        }
    }

    fn expect_or_abort<S>(self, message: S) -> T
    where
        S: AsRef<str>,
    {
        match self {
            Ok(v) => v,
            Err(e) => {
                e.emit();
                abort!(Span::call_site(), message.as_ref())
            }
        }
    }
}
