//! Adapted from https://github.com/bodil/typed-html/blob/master/typed-html/src/types/class.rs

use crate::{
    error::Error,
    result::Result,
};
pub use rust_liveview_util::{
    AsRef,
    Deref,
    Display,
};
use std::{
    convert::TryFrom,
    str::FromStr,
};

/// A valid CSS class.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deref, AsRef, Debug, Display)]
pub struct Class(String);

impl FromStr for Class {
    type Err = Error;
    fn from_str(id: &str) -> Result<Self> {
        let mut chars = id.chars();
        match chars.next() {
            None => return Err("A class name cannot be empty".into()),
            Some(c) if !c.is_alphabetic() => {
                return Err("A class name must start with an alphabetic character".into())
            }
            _ => (),
        }
        for c in chars {
            if !c.is_alphanumeric() && c != '_' && c != '-' && c != '.' {
                return Err(
                    "A class name can only contain alphanumerics, dash, dot and underscore".into(),
                );
            }
        }
        Ok(Self(id.to_owned()))
    }
}

impl<'a> TryFrom<&'a str> for Class {
    type Error = Error;
    fn try_from(id: &'a str) -> Result<Self> {
        Self::from_str(id)
    }
}
