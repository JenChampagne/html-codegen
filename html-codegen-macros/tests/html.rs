use html_codegen_macros::html_minified;

#[test]
fn single_regular_tag() {
    let output: String = format!("{}", html_minified! { <div> </div> });

    assert_eq!(output.as_str(), "<div></div>");
}

#[test]
fn single_regular_tag_with_literal_child() {
    let output: String = format!("{}", html_minified! { <div> {"Hello world!"} </div> });

    assert_eq!(output.as_str(), "<div>Hello world!</div>");
}

#[test]
fn single_regular_tag_with_value_child() {
    let message = "Hello world!";

    let output: String = format!("{}", html_minified! { <div> {message} </div> });

    assert_eq!(output.as_str(), "<div>Hello world!</div>");
}

#[test]
fn single_regular_tag_with_void_attribute() {
    let output: String = format!("{}", html_minified! { <div checked></div> });

    assert_eq!(output.as_str(), "<div checked></div>");
}

#[test]
fn single_regular_tag_with_regular_attribute() {
    let output: String = format!("{}", html_minified! { <div id={"z1"}></div> });

    assert_eq!(output.as_str(), r#"<div id="z1"></div>"#);
}

#[test]
fn nested_regular_tags_with_regular_attribute() {
    let output: String = format!(
        "{}",
        html_minified! { <div id={"a"}><div id={"b"}></div></div> }
    );

    assert_eq!(output.as_str(), r#"<div id="a"><div id="b"></div></div>"#);
}

#[test]
fn regular_tags_with_regular_attribute_with_raw_child() {
    let output: String = format!("{}", html_minified! { <div id={"a"}>{"Hello"}</div> });

    assert_eq!(output.as_str(), r#"<div id="a">Hello</div>"#);
}

#[test]
fn regular_tags_with_regular_attribute_with_raw_escaped_child() {
    pub struct EscapeHtmlEntities(&'static str);
    impl core::fmt::Display for EscapeHtmlEntities {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for c in self.0.chars() {
                match c {
                    '>' => write!(f, "&gt;")?,
                    '<' => write!(f, "&lt;")?,
                    '"' => write!(f, "&quot;")?,
                    '&' => write!(f, "&amp;")?,
                    '\'' => write!(f, "&apos;")?,
                    c => write!(f, "{c}")?,
                };
            }
            Ok(())
        }
    }
    let output: String = format!(
        "{}",
        html_minified! { <div id={"a"}>{EscapeHtmlEntities("<abc></abc>")}</div> }
    );

    pretty_assertions::assert_eq!(
        output.as_str(),
        r#"<div id="a">&lt;abc&gt;&lt;/abc&gt;</div>"#
    );
}
