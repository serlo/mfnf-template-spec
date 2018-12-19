pub mod markdown;
mod spec;

pub use crate::spec::*;

#[test]
fn generate_definition_doc() {
    let spec = spec_of(":Mathe für Nicht-Freaks: Vorlage:Gruppenaufgabe")
        .expect("Could not find template spec!");
    println!(
        "{}",
        &markdown::template_description(&spec, 1).expect("could not format markdown")
    );
}
