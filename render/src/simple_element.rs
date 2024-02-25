use crate::html_escaping::escape_html;
use crate::{Raw, Render};
use ordered_hashmap::OrderedHashMap;
use std::borrow::Cow;
use std::fmt::{Result, Write};

#[derive(Clone, Debug)]
pub enum AV<'a> {
    None,
    Some(Cow<'a, str>),
    SomeRaw(Raw<'a>),
    Short,
}

impl<'a> From<AV<'a>> for Option<Cow<'a, str>> {
    fn from(value: AV<'a>) -> Self {
        match value {
            AV::None => None,

            AV::Some(x) => Some(x),

            AV::SomeRaw(Raw(x)) => Some(Cow::Borrowed(x)),

            AV::Short => None,
        }
    }
}

pub trait ToAttribute<'a> {
    fn from_value(self) -> AV<'a>;
}

impl<'a> ToAttribute<'a> for Option<Cow<'a, str>> {
    fn from_value(self) -> AV<'a> {
        match self {
            None => AV::None,
            Some(x) => AV::Some(x),
        }
    }
}

impl<'a> ToAttribute<'a> for bool {
    fn from_value(self) -> AV<'a> {
        if self {
            AV::Short
        } else {
            AV::None
        }
    }
}

impl<'a> ToAttribute<'a> for () {
    fn from_value(self) -> AV<'a> {
        AV::None
    }
}

impl<'a> ToAttribute<'a> for String {
    fn from_value(self) -> AV<'a> {
        AV::Some(Cow::Owned(self))
    }
}

impl<'a> ToAttribute<'a> for &'a str {
    fn from_value(self) -> AV<'a> {
        AV::Some(Cow::Borrowed(self))
    }
}

impl<'a> ToAttribute<'a> for Option<&'a str> {
    fn from_value(self) -> AV<'a> {
        match self {
            None => AV::None,
            Some(x) => AV::Some(Cow::Borrowed(x)),
        }
    }
}

impl<'a> ToAttribute<'a> for Option<String> {
    fn from_value(self) -> AV<'a> {
        match self {
            None => AV::None,
            Some(x) => AV::Some(Cow::Owned(x)),
        }
    }
}

impl<'s> ToAttribute<'s> for Raw<'s> {
    fn from_value(self) -> crate::AV<'s> {
        AV::SomeRaw(self)
    }
}

macro_rules! impl_primitive {
    [$($num: ty),+] => {
        $(
            impl<'a> ToAttribute<'a> for $num {
                fn from_value(self) -> AV<'a> {
                    AV::Some(Cow::Owned(self.to_string()))
                }
            }
        )+
    };
}
impl_primitive![u8, u16, u32, u64, i8, i16, i32, i64, f32, f64];

impl<'a> ToAttribute<'a> for Cow<'a, str> {
    fn from_value(self) -> AV<'a> {
        AV::Some(self)
    }
}

type Attributes<'a> = Option<OrderedHashMap<&'a str, AV<'a>>>;

/// Simple HTML element tag
#[derive(Debug, Clone)]
pub struct SimpleElement<'a, T: Render> {
    /// the HTML tag name, like `html`, `head`, `body`, `link`...
    pub tag_name: &'a str,
    pub attributes: Attributes<'a>,
    pub contents: Option<T>,
}

fn write_attributes<'a, W: Write>(attributes: Attributes<'a>, writer: &mut W) -> Result {
    match attributes {
        None => Ok(()),
        Some(attributes) => {
            for (key, maybe_value) in attributes {
                match maybe_value {
                    AV::Some(value) => {
                        write!(writer, " {key}=\"")?;
                        escape_html(&value, writer)?;
                        write!(writer, "\"")?;
                    }

                    AV::SomeRaw(Raw(value)) => {
                        write!(writer, " {key}=\"")?;
                        write!(writer, "{value}")?;
                        write!(writer, "\"")?;
                    }

                    AV::Short => {
                        write!(writer, " {key}")?;
                    }

                    AV::None => {}
                }
            }
            Ok(())
        }
    }
}

impl<T: Render> Render for SimpleElement<'_, T> {
    fn render_into<W: Write>(self, writer: &mut W) -> Result {
        match self.contents {
            None => {
                write!(writer, "<{}", self.tag_name)?;
                write_attributes(self.attributes, writer)?;

                match self.tag_name {
                    "area" | "base" | "br" | "col" | "embed" | "hr" | "img" | "input" | "link"
                    | "meta" | "param" | "source" | "track" | "wbr" => {
                        // void element, can be self-closing
                        write!(writer, "/>")
                    }
                    _ => {
                        write!(writer, "></{}>", self.tag_name)
                    }
                }
            }
            Some(renderable) => {
                write!(writer, "<{}", self.tag_name)?;
                write_attributes(self.attributes, writer)?;
                write!(writer, ">")?;
                renderable.render_into(writer)?;
                write!(writer, "</{}>", self.tag_name)
            }
        }
    }
}
