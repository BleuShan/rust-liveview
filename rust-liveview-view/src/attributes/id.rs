//! Adapted from https://github.com/bodil/typed-html/blob/master/typed-html/src/types/id.rs

use crate::{
    error::Error,
    result::Result,
};
use rust_liveview_common::{
    AsRef,
    Deref,
    Display,
};
use std::{
    convert::TryFrom,
    str::FromStr,
};

/// A valid HTML/SVG id.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deref, AsRef, Debug, Display)]
pub struct Id(String);

impl FromStr for Id {
    type Err = Error;
    fn from_str(id: &str) -> Result<Self> {
        let mut chars = id.chars();
        match chars.next() {
            None => return Err("An ID cannot be empty".into()),
            Some(c) if !c.is_alphabetic() => {
                return Err("An ID must start with an alphabetic character".into())
            }
            _ => (),
        }
        for c in chars {
            if !c.is_alphanumeric() && c != '_' && c != '-' && c != '.' {
                return Err(
                    "An ID can only contain alphanumerics, dash, dot and underscore".into(),
                );
            }
        }
        Ok(Self(id.to_owned()))
    }
}

impl<'a> TryFrom<&'a str> for Id {
    type Error = Error;
    fn try_from(id: &'a str) -> Result<Self> {
        Self::from_str(id)
    }
}
