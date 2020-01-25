//! HTML elements.

use super::attributes::*;
use crate::prelude::*;

declare_elements! {
    Html {
        xmlns: Uri,
    },
    Head {},
    Body {}
}
