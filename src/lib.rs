#[macro_use]
extern crate mwparser_utils_derive;
#[macro_use]
extern crate mwparser_utils;
extern crate mediawiki_parser;
#[macro_use]
extern crate serde_derive;

mod spec;
pub mod markdown;

pub use spec::*;

#[test]
fn generate_definition_doc() {
    let spec = spec_of(":Mathe für Nicht-Freaks: Vorlage:Gruppenaufgabe")
        .expect("Could not find template spec!");
    println!("{}", &markdown::template_description(&spec, 1));
}
