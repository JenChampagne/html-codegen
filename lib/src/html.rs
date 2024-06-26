//! HTML utilities

use crate::Render;
use std::fmt::{Result, Write};

/// HTML 5 doctype declaration
///
/// ```rust
/// # use pretty_assertions::assert_eq;
/// # use html_codegen::html::HTML5Doctype;
/// # use html_codegen::html;
/// # let result =
/// html! {
///     <>
///         <HTML5Doctype />
///         <html>
///             <body />
///         </html>
///     </>
/// }.unwrap();
/// # assert_eq!(result, "<!DOCTYPE html><html><body></body></html>");
/// ```
#[derive(Debug, Clone)]
pub struct HTML5Doctype;

impl Render for HTML5Doctype {
    fn render_into<W: Write>(self, writer: &mut W) -> Result {
        write!(writer, "<!DOCTYPE html>")
    }
}
