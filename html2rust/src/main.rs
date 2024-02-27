mod args;

use html_parser::Dom;
use html_parser::{ElementVariant, Node};
use regex::Regex;
use std::collections::HashSet;

fn raw_string_escape(input: &str) -> (String, String) {
    let re = Regex::new(r#""(#*)"#).unwrap();
    let quote_kinds = re
        .find_iter(input)
        .map(|found| found.as_str())
        .collect::<HashSet<_>>();

    if quote_kinds.len() == 0 {
        return (String::from(r#"""#), String::from(r#"""#));
    }

    let mut octothorpes = (String::from("r#"), String::from(r##""#"##));
    loop {
        if !quote_kinds.contains(octothorpes.1.as_str()) {
            octothorpes.0.push('"');
            return octothorpes;
        }

        octothorpes.0.push('#');
        octothorpes.1.push('#');
    }
}

fn recurse_elements(nodes: Vec<Node>, prefix: &str) -> String {
    let mut output = String::new();
    for node in nodes {
        match node {
            Node::Text(text) => {
                let quotes = raw_string_escape(&text);

                output.push_str(prefix);
                output.push('{');
                output.push_str(&quotes.0);
                output.push_str(&text);
                output.push_str(&quotes.1);
                output.push_str("}\n");
            }
            Node::Element(element) => {
                output.push_str(prefix);
                output.push_str("<");
                output.push_str(&element.name);

                if let Some(id) = element.id {
                    output.push_str(r#" id={""#);
                    output.push_str(&id);
                    output.push_str(r#""}"#);
                }

                if !element.classes.is_empty() {
                    output.push_str(r#" class={""#);
                    output.push_str(&element.classes.join(" "));
                    output.push_str(r#""}"#);
                }

                for (key, value) in element.attributes {
                    output.push(' ');
                    output.push_str(&key);
                    if let Some(value) = value {
                        let quotes = raw_string_escape(&value);

                        output.push_str("={");
                        output.push_str(&quotes.0);
                        output.push_str(&value);
                        output.push_str(&quotes.1);
                        output.push_str("}");
                    } else {
                        output.push_str("={true}");
                    }
                }

                // todo: Confirm whether or not a void element can have children
                // and if so, how should that be handled here?
                if let ElementVariant::Void = element.variant {
                    output.push_str(" />\n");
                } else {
                    output.push_str(">\n");
                }

                let new_prefix = format!("{prefix}    ");
                output.push_str(&recurse_elements(element.children, &new_prefix));

                if let ElementVariant::Normal = element.variant {
                    output.push_str(prefix);
                    output.push_str("</");
                    output.push_str(&element.name);
                    output.push_str(">\n");
                }
            }
            Node::Comment(comment) => {
                let indented_comment = comment
                    .lines()
                    .map(|line| format!("{prefix}// {line}").trim_end().to_string())
                    .collect::<Vec<String>>()
                    .join("\n");

                output.push_str(&indented_comment);
                output.push('\n');
            }
        }
    }
    output
}

fn main() {
    dotenvy::dotenv().ok();
    let args = args::Args::load();

    let plain = if let Some(input_file_path) = args.input_file_path {
        std::fs::read_to_string(input_file_path).unwrap()
    } else {
        std::io::stdin()
            .lines()
            .collect::<Result<Vec<String>, std::io::Error>>()
            .unwrap()
            .join("\n")
    };

    let html = Dom::parse(&plain).unwrap();

    if !html.errors.is_empty() {
        println!("Errors: {:#?}", html.errors);
        return;
    }

    let mut output = String::with_capacity(plain.len());

    if args.no_mangle {
        output.push_str("#[no_mangle]\n");
    }

    let function_declaration = format!(
        "pub fn {}() -> Result<String, core::fmt::Error> {{\n    render::html!{{\n",
        args.function_name.as_deref().unwrap_or("html")
    );
    output.push_str(function_declaration.as_str());

    if html
        .children
        .iter()
        .filter(|child| matches!(child, &Node::Text(_) | &Node::Element(_)))
        .count()
        != 1
    {
        output.push_str("        <>\n");
        output.push_str(&recurse_elements(html.children, "            "));
        output.push_str("        </>\n");
    } else {
        output.push_str(&recurse_elements(html.children, "        "));
    }
    output.push_str("    }\n}");

    println!("{output}");
}
