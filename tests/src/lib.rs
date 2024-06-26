#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("ui/fail/*.rs");
}

#[test]
fn maintains_valid_urls_with_raw_attribute() {
    use html_codegen::{html, raw};
    use pretty_assertions::assert_eq;

    let value = html! { <a href={raw!("https://example.com/home?a=1&b=2&c=3")}>{"x"}</a> }.unwrap();

    assert_eq!(
        value,
        r#"<a href="https://example.com/home?a=1&b=2&c=3">x</a>"#
    );
}

#[test]
fn works_with_dashes() {
    use pretty_assertions::assert_eq;

    let value =
        html_codegen::html! { <div data-id={"myid"} hx-get={"x"} checked={false} unchecked={true} /> }
            .unwrap();
    assert_eq!(value, r#"<div data-id="myid" hx-get="x" unchecked></div>"#);
}

#[test]
fn works_with_raw() {
    use html_codegen::{html, raw};
    use pretty_assertions::assert_eq;

    let actual = html! {
        <div>{raw!("<Hello />")}</div>
    }
    .unwrap();

    assert_eq!(actual, "<div><Hello /></div>");
}

#[test]
fn works_with_htmx_ident() {
    use pretty_assertions::assert_eq;

    let actual = html_codegen::html! {
        <input hx-get={"url"} />
    }
    .unwrap();

    assert_eq!(actual, r#"<input hx-get="url"/>"#);
}

#[test]
fn works_with_raw_ident() {
    use pretty_assertions::assert_eq;

    let actual = html_codegen::html! {
        <input r#type={"text"} />
    }
    .unwrap();

    assert_eq!(actual, r#"<input type="text"/>"#);
}

#[test]
fn works_with_keywords() {
    use html_codegen::html;
    use pretty_assertions::assert_eq;

    assert_eq!(
        html! { <input type={"text"} /> }.unwrap(),
        r#"<input type="text"/>"#
    );
    assert_eq!(
        html! { <label for={"me"} /> }.unwrap(),
        r#"<label for="me"></label>"#
    );
}

#[test]
fn selfclosing_void_element() {
    use html_codegen::html;
    use pretty_assertions::assert_eq;

    assert_eq!(html! { <hr /> }.unwrap(), r#"<hr/>"#);
}

#[test]
fn element_ordering() {
    use html_codegen::html;
    use pretty_assertions::assert_eq;

    let actual = html! {
      <ul>
        <li>{"1"}</li>
        <li>{"2"}</li>
        <li>{"3"}</li>
      </ul>
    }
    .unwrap();

    assert_eq!(actual, "<ul><li>1</li><li>2</li><li>3</li></ul>");

    let deep = html! {
      <div>
        <h1>{"A list"}</h1>
        <hr />
        <ul>
          <li>{"1"}</li>
          <li>{"2"}</li>
          <li>{"3"}</li>
        </ul>
      </div>
    }
    .unwrap();

    assert_eq!(
        deep,
        "<div><h1>A list</h1><hr/><ul><li>1</li><li>2</li><li>3</li></ul></div>"
    );
}

#[test]
fn childless_non_selfclosing_tag() {
    use html_codegen::html;
    use pretty_assertions::assert_eq;

    let actual = html! {
        <textarea></textarea>
    }
    .unwrap();

    assert_eq!(actual, "<textarea></textarea>");

    let actual = html! {
        <script></script>
    }
    .unwrap();

    assert_eq!(actual, "<script></script>");
}

#[test]
fn some_none() {
    use html_codegen::{component, html, rsx};
    use pretty_assertions::assert_eq;

    #[component]
    fn Answer(a: i8) {
        rsx! {
          <>
            {match a {
              42 => Some("Yes"),
              _ => None,
            }}
          </>
        }
    }

    assert_eq!(html! { <Answer a={42} /> }.unwrap(), "Yes");
    assert_eq!(html! { <Answer a={44} /> }.unwrap(), "");
}

#[test]
fn owned_string() {
    use html_codegen::{component, html, rsx};
    use pretty_assertions::assert_eq;

    #[component]
    fn Welcome<'kind, 'name>(kind: &'kind str, name: &'name str) {
        rsx! {
            <h1 class={format!("{kind}-title")}>
                {format!("Hello, {name}")}
            </h1>
        }
    }

    assert_eq!(
        html! { <Welcome kind={"alien"} name={"Yoda"} /> }.unwrap(),
        r#"<h1 class="alien-title">Hello, Yoda</h1>"#
    );
}

#[test]
fn cow_str() {
    use html_codegen::html;
    use pretty_assertions::assert_eq;
    use std::borrow::Cow;

    let owned1 = "Borrowed from owned".to_owned();
    let owned2 = "Owned".to_owned();

    assert_eq!(
        html! {
            <div>
                <p>{Cow::Borrowed("Static")}</p>
                <p>{Cow::<'_, str>::Borrowed(&owned1)}</p>
                <p>{Cow::<'_, str>::Owned(owned2)}</p>
            </div>
        }
        .unwrap(),
        r#"<div><p>Static</p><p>Borrowed from owned</p><p>Owned</p></div>"#,
    );
}

#[test]
fn number() {
    use html_codegen::html;
    use pretty_assertions::assert_eq;

    let num = 42;

    assert_eq!(html! { <p>{num}</p> }.unwrap(), "<p>42</p>")
}

#[test]
fn vec() {
    use html_codegen::html;
    use pretty_assertions::assert_eq;

    let list = vec!["Mouse", "Rat", "Hamster"];

    assert_eq!(
        html! {
            <ul>
                {
                    list
                        .into_iter()
                        .map(|text| html_codegen::rsx! { <li>{text}</li> })
                        .collect::<Vec<_>>()
                }
            </ul>
        }
        .unwrap(),
        "<ul><li>Mouse</li><li>Rat</li><li>Hamster</li></ul>"
    )
}

mod kaki {
    // A simple HTML 5 doctype declaration
    use html_codegen::html::HTML5Doctype;
    use html_codegen::{
        // A macro to create components
        component,
        // A macro to compose components in JSX fashion
        rsx,
        // A trait for custom components
        Render,
    };

    // This can be any layout we want
    #[component]
    fn Page<'a, Children: Render>(title: &'a str, children: Children) {
        rsx! {
          <>
            <HTML5Doctype />
            <html>
              <head><title>{title}</title></head>
              <body hx-boost={"true"} hx-swap={"innerHTML"} checked={false}>
                {children}
              </body>
            </html>
          </>
        }
    }

    #[test]
    fn test() {
        use pretty_assertions::assert_eq;
        let actual = html_codegen::html! {
          <Page title={"Home"}>
            {format!("Welcome, {}", "Gal")}
          </Page>
        }
        .unwrap();
        let expected = concat!(
            "<!DOCTYPE html>",
            "<html>",
            "<head><title>Home</title></head>",
            r#"<body hx-boost="true" hx-swap="innerHTML">"#,
            "Welcome, Gal",
            "</body>",
            "</html>"
        );
        assert_eq!(actual, expected);
    }

    #[test]
    fn externals_test() {
        use crate::other::ExternalPage;
        use pretty_assertions::assert_eq;

        let actual = html_codegen::html! {
          <ExternalPage title={"Home"} subtitle={"Foo"}>
            {format!("Welcome, {}", "Gal")}
          </ExternalPage>
        }
        .unwrap();

        let expected = concat!(
            "<!DOCTYPE html>",
            "<html>",
            "<head><title>Home</title></head>",
            "<body>",
            "<h1>Foo</h1>",
            "Welcome, Gal",
            "</body>",
            "</html>"
        );
        assert_eq!(actual, expected);
    }
}

/// ## Other
///
/// Module for testing component visibility when imported from other modules.

mod other {
    use html_codegen::html::HTML5Doctype;
    use html_codegen::{component, rsx, Render};

    #[component]
    pub fn ExternalPage<'title, 'subtitle, Children: Render>(
        title: &'title str,
        subtitle: &'subtitle str,
        children: Children,
    ) {
        rsx! {
            <>
              <HTML5Doctype />
              <html>
                <head><title>{title}</title></head>
                <body>
                  <h1>{subtitle}</h1>
                  {children}
                </body>
              </html>
            </>
        }
    }
}
