//! Simple crate of HTML producing template functions.
//! These are separate from the main application so that it can be
//! rapdily recompiled without needing to recompile an entire large binary.
//!
//! Note that since this crate is sometimes loaded dynamically at run-time,
//! exported functions must have deterministic names that can be referenced.
//! For this reason, all functions intended for external use via dynamic
//! run-time loading must have a #[no_mangle] declaration.
//!
//! It is recommended, but not required, to avoid using the standard library
//! or other external crates where possible. This is both to prevent linking
//! issues as well as to keep the compile time of this crate low.
//!
//! Given that, it is also recommended that custom types are defined here,
//! however you may want to avoid deriving parsing functionality that is
//! slow to compile.
//!
//! For example, I might make a custom struct but not implement
//! `serde::Serialize` for it since that adds a lot of compile time here.
//! Instead you could define the same struct in both places with the slower
//! building application have the derived translations.
//!
//! ```rs
//! // html crate
//! pub struct Form { name: String }
//! ```
//!
//! ```rs
//! // api crate
//! use example_html::Form as ExternalForm;
//!
//! #[derive(Clone, Debug, Serialize, Deserialize)]
//! pub struct Form { name: String }
//!
//! impl From<Form> for ExternalForm {
//!     fn from(api_value: Form) -> Self {
//!         ExternalForm {
//!             name: api_value.name,
//!         }
//!     }
//! }
//!
//! fn route() {
//!     let form = Form { name: String::from("Alice") };
//!     example_html::some_template(form.into());
//! }
//! ```
//!
//! See the `api-*` folders in this repository for fully functional examples.

use core::fmt::Error;
use html_codegen::html_format;

#[no_mangle]
pub fn always_error() -> Result<String, Error> {
    Err(Error)
}

#[no_mangle]
pub fn never_error() -> String {
    html_format! {
        <div class={"400-green"}>{"Cool as a cucumber#"}</div>
    }
}

#[no_mangle]
pub fn greeting() -> Result<String, Error> {
    Ok(html_format! {
      <>
        <h1>{"Hello!"}</h1>
        <hr />
        <ul>
          <li>{"Welcome friends."}</li>
        </ul>
      </>
    })
}

#[no_mangle]
pub fn exclusive_ref_input<'a>(input: &'a mut String) {
    input.clear();
    input.push_str(&html_format! {
        <p>{"Mutation baby"}</p>
    });
}
