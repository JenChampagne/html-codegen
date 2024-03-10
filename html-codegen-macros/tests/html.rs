use html_codegen_macros::html_format;

#[test]
fn single_regular_tag() {
    let output: String = html_format! { <div> </div> };

    assert_eq!(output.as_str(), "<div></div>");
}

#[test]
fn single_regular_tag_with_literal_child() {
    let output: String = html_format! { <div> {"Hello world!"} </div> };

    assert_eq!(output.as_str(), "<div>Hello world!</div>");
}

#[test]
fn single_regular_tag_with_value_child() {
    let message = "Hello world!";

    let output: String = html_format! { <div> {message} </div> };

    assert_eq!(output.as_str(), "<div>Hello world!</div>");
}

#[test]
fn single_regular_tag_with_void_attribute() {
    let output: String = html_format! { <div checked></div> };

    assert_eq!(output.as_str(), "<div checked></div>");
}

#[test]
fn single_regular_tag_with_regular_attribute() {
    let output: String = html_format! { <div id={"z1"}></div> };

    assert_eq!(output.as_str(), r#"<div id="z1"></div>"#);
}

#[test]
fn nested_regular_tags_with_regular_attribute() {
    let output: String = html_format! { <div id={"a"}><div id={"b"}></div></div> };

    assert_eq!(output.as_str(), r#"<div id="a"><div id="b"></div></div>"#);
}

#[test]
fn regular_tags_with_regular_attribute_with_raw_child() {
    let output: String = html_format! { <div id={"a"}>{"Hello"}</div> };

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
    let output: String = html_format! { <div id={"a"}>{EscapeHtmlEntities("<abc></abc>")}</div> };

    pretty_assertions::assert_eq!(
        output.as_str(),
        r#"<div id="a">&lt;abc&gt;&lt;/abc&gt;</div>"#
    );
}

#[test]
fn void_tag_root_to_group_children() {
    let output: String = html_format! {
      <>
        <h1>{"Hola"}</h1>
        <hr />
      </>
    };

    assert_eq!(output.as_str(), "<h1>Hola</h1><hr />");
}

#[test]
fn void_tags_to_group_children() {
    let output: String = html_format! {
      <>
        <h1>{"Hola"}</h1>
        <hr />
        <>
            <h2>{"Section"}</h2>
            <span>{"Hi."}</span>
        </>
      </>
    };

    assert_eq!(
        output.as_str(),
        "<h1>Hola</h1><hr /><h2>Section</h2><span>Hi.</span>"
    );
}

#[test]
fn nested_tags_with_variable_input() {
    let value = "Hola";

    let output: String = html_format! {
      <div>
        <h1>{value}</h1>
        <hr />
      </div>
    };

    assert_eq!(output.as_str(), "<div><h1>Hola</h1><hr /></div>");
}
