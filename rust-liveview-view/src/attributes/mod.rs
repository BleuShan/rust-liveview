//! HTML attributes values.

pub use rust_liveview_util::{
    http::Uri,
    language_tags::LanguageTag,
    mime::Mime,
};

mod class;
mod id;
mod spacedset;

pub use class::Class;
pub use id::Id;
pub use spacedset::SpacedSet;
