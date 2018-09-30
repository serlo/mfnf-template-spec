//! CommonMark rendering of the template specification.

use spec_meta;
use std::fmt::Write;

/// Renders a markdown version of a template specification.
///
/// The `heading_depth` is added to the depth of the headings produced.
pub fn template_description(
    template: &spec_meta::TemplateSpec,
    heading_depth: usize
) -> String {
    let mut out = format!(
        "{} Documentation for `{}` [{}]\n\n",
        "#".repeat(heading_depth + 1),
        template.default_name(),
        &format!("{:?}", &template.format).to_lowercase()
    );

    let names = template.names.join(", ");
    writeln!(&mut out, "Other names: *{}*\n", &names);
    writeln!(&mut out, "{}\n", &template.description);
    writeln!(&mut out, "{} Template Attributes:\n", "#".repeat(heading_depth + 2));

    for attribute in &template.attributes {
        let mut alt_names = &attribute.names
            .iter()
            .filter(|n| n != &attribute.default_name())
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        writeln!(
            &mut out,
            "  - `{}` {}[**{}**][**{}**]: \n",
            attribute.default_name(),
            if alt_names.is_empty() {
                String::new()
            } else {
                format!("*({})* ", &alt_names)
            },
            &format!("{:?}", &attribute.priority).to_lowercase(),
            &attribute.predicate_name,
        );
        let description = attribute.description
            .split("\n")
            .map(|s| {
                let mut new = " ".repeat(4);
                new.push_str(s);
                new
            })
            .collect::<Vec<String>>()
            .join("\n");

        writeln!(&mut out, "{}\n", &description);
    }
    out
}

