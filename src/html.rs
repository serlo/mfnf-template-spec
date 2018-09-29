//! HTML rendering of the template specification.

use spec_meta;
use pulldown_cmark::{html, Parser};

/// Renders a html version of a template specification.
pub fn template_description(template: &spec_meta::TemplateSpec) -> String {
    let mut output = String::new();
    output.push_str("<div class=\"template-doc\">");
    output.push_str("<div class=\"template-title\">");
    output.push_str("Documentation for ");
    output.push_str(template.default_name());
    output.push_str("<span class=\"template-format\">");
    output.push_str(&format!("{:?}", &template.format));
    output.push_str("</span></div>");
    output.push_str("<span class=\"template-aka\">Also known as ");
    output.push_str(&template.names.join(", "));
    output.push_str("</span><br>");
    output.push_str("<div class=\"template-description\">");
    let parser = Parser::new(&template.description);
    html::push_html(&mut output, parser);
    output.push_str("</div>");
    output.push_str("<div class=\"template-attributes\">Attributes:<ul>");

    for attribute in &template.attributes {
        output.push_str("<li><div class=\"attribute-doc\">");
        output.push_str("<div class=\"attribute-title\">");
        output.push_str(attribute.default_name());
        output.push_str("<span class=\"attribute-priority\">");
        output.push_str(&format!("{:?}", &attribute.priority));
        output.push_str("</span></div>");
        output.push_str("<span class=\"attribute-aka\">Also known as ");
        output.push_str(&attribute.names.join(", "));
        output.push_str("</span><br>");
        output.push_str("<span class=\"attribute-predicate\">");
        output.push_str(&attribute.predicate_name);
        output.push_str("</span> must be fulfilled!<br>");
        output.push_str("<div class=\"attribute-description\">");
        let parser = Parser::new(&attribute.description);
        html::push_html(&mut output, parser);
        output.push_str("</div></li>");
    }

    output.push_str("</ul></div>");
    output.push_str("</div>");
    output
}
