pub use mime::*;
use mime_guess::{
    from_ext,
    from_path,
};
use std::path::Path;

#[inline]
pub fn path_to_mime<P>(path: P) -> Mime
where
    P: AsRef<Path>,
{
    from_path(path).first_or_octet_stream()
}

#[inline]
pub fn file_extension_to_mime<S>(extension: S) -> Mime
where
    S: AsRef<str>,
{
    from_ext(extension.as_ref()).first_or_octet_stream()
}
