//! The fragment component

use crate::Render;
use std::fmt::{Result, Write};

/// A top-level root component to combine a same-level components
/// in a RSX fashion
///
/// ```rust
/// # use pretty_assertions::assert_eq;
/// # use html_codegen_macros::html;
/// let result = html! {
///     <>
///         <a />
///         <b />
///     </>
/// }.unwrap();
/// assert_eq!(result, "<a></a><b></b>");
/// ```
#[derive(Debug, Clone)]
pub struct Fragment<T: Render> {
    pub children: T,
}

impl<T: Render> Render for Fragment<T> {
    fn render_into<W: Write>(self, writer: &mut W) -> Result {
        self.children.render_into(writer)
    }
}
