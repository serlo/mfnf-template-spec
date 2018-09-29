#[macro_use]
extern crate mwparser_utils_derive;
#[macro_use]
extern crate mwparser_utils;
extern crate mediawiki_parser;
#[macro_use]
extern crate serde_derive;
extern crate pulldown_cmark;

mod spec;
pub mod html;

pub use spec::*;
