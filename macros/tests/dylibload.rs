use html_codegen::dylibload::{dylibload, Error};
use pretty_assertions::assert_eq;

#[dylibload(example_html)]
pub fn always_error() -> Result<String, core::fmt::Error> {}

#[dylibload(example_html)]
pub fn never_error() -> String {}

#[dylibload(example_html)]
pub fn greeting() -> Result<String, core::fmt::Error> {}

#[dylibload(example_html)]
pub fn exclusive_ref_input<'a>(input: &'a mut String) {}

#[test]
fn simple_always_error() {
    let html = always_error().expect("library loaded");

    assert_eq!(html, Err(core::fmt::Error));
}

#[test]
fn simple_never_error() {
    let html = never_error().expect("library loaded");

    assert_eq!(html, r#"<div class="400-green">Cool as a cucumber.</div>"#);
}

#[test]
fn simple_greeting() {
    let html = greeting()
        .expect("library loaded")
        .expect("valid html generated");

    assert_eq!(
        html,
        "<h1>Hello!</h1><hr /><ul><li>Welcome friends.</li></ul>"
    );
}

#[test]
fn simple_exclusive_ref_input() {
    let mut html = String::new();
    exclusive_ref_input(&mut html).expect("library loaded");

    assert_eq!(html, "<p>Mutation baby</p>");
}
