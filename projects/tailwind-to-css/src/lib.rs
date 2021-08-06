// #![feature(type_alias_impl_trait)]
// #![feature(box_syntax)]
#![forbid(missing_debug_implementations)]
#![forbid(missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://upload.wikimedia.org/wikipedia/commons/d/d5/Tailwind_CSS_Logo.svg")]
#![doc(html_favicon_url = "https://upload.wikimedia.org/wikipedia/commons/d/d5/Tailwind_CSS_Logo.svg")]

extern crate core;

// mod css;
mod macros;
mod modules;
mod systems;
mod traits;

pub use self::{
    modules::{
        accessibility::*, background::*, borders::*, effects::*, filters::*, flexbox::*, interactivity::*, layouts::*,
        sizing::*, spacing::*, tables::*, transforms::*, transition::*, typography::*,
    },
    systems::{breakpoints::*, builder::*, colors::*, css_global::*, fonts::*, instruction::*, length::*, preflight::*},
    traits::{CssAttribute, ParsedItem, ParsedList, TailwindInstance, TailwindObject},
};
pub use tailwind_error::{Result, TailwindError, TailwindErrorKind};
