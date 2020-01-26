//! Adapted from https://github.com/bodil/typed-html/blob/master/typed-html/src/types/spacedset.rs

use rust_liveview_common::{
    Deref,
    DerefMut,
};
use std::{
    collections::BTreeSet,
    convert::{
        TryFrom,
        TryInto,
    },
    fmt::{
        self,
        Debug,
        Display,
        Formatter,
    },
    iter::FromIterator,
    str::FromStr,
};

/// A space separated set of unique values.
#[derive(Clone, PartialEq, Eq, Hash, Default, Deref, DerefMut, Debug)]
pub struct SpacedSet<A: Ord>(BTreeSet<A>);

impl<A: Ord> SpacedSet<A> {
    /// Add a value to the `SpacedSet`, converting it as necessary.
    pub fn try_add<T: TryInto<A>>(&mut self, value: T) -> Result<bool, <T as TryInto<A>>::Error> {
        Ok(self.0.insert(value.try_into()?))
    }
}

impl<A: Ord> FromIterator<A> for SpacedSet<A> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = A>,
    {
        SpacedSet(iter.into_iter().collect())
    }
}

impl<'a, A: 'a + Ord + Clone> FromIterator<&'a A> for SpacedSet<A> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = &'a A>,
    {
        SpacedSet(iter.into_iter().cloned().collect())
    }
}

impl<'a, A: Ord + FromStr> FromStr for SpacedSet<A>
where
    <A as FromStr>::Err: Debug,
{
    type Err = <A as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result: Result<Vec<A>, Self::Err> =
            s.split_whitespace().map(|s| FromStr::from_str(s)).collect();
        result.map(Self::from_iter)
    }
}

impl<'a, A> TryFrom<&'a str> for SpacedSet<A>
where
    A: Ord + FromStr,
{
    type Error = <A as FromStr>::Err;
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        s.split_whitespace().map(FromStr::from_str).collect()
    }
}

impl<A> Display for SpacedSet<A>
where
    A: Ord + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        let mut it = self.0.iter().peekable();
        while let Some(item) = it.next() {
            Display::fmt(item, f)?;
            if it.peek().is_some() {
                Display::fmt(" ", f)?;
            }
        }
        Ok(())
    }
}
