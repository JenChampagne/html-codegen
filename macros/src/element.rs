use crate::children::Children;
use crate::element_attribute::ElementAttribute;
use crate::element_attributes::ElementAttributes;
use crate::tags::{ClosingTag, OpenTag};
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream, Result};

pub struct Element {
    pub name: syn::Path,
    pub attributes: ElementAttributes,
    pub children: Children,
    pub is_self_closing: bool,
}

impl Parse for Element {
    fn parse(input: ParseStream) -> Result<Self> {
        let open_tag = input.parse::<OpenTag>()?;

        let children = if open_tag.self_closing {
            Children::default()
        } else {
            let children = input.parse::<Children>()?;
            let closing_tag = input.parse::<ClosingTag>()?;
            closing_tag.validate(&open_tag);
            children
        };

        Ok(Element {
            name: open_tag.name,
            attributes: open_tag.attributes,
            children,
            is_self_closing: open_tag.self_closing,
        })
    }
}

impl Element {
    pub fn is_custom_element(&self) -> bool {
        match self.name.get_ident() {
            None => true,
            Some(ident) => {
                let name = ident.to_string();
                let first_letter = name.get(0..1).unwrap();
                first_letter.to_uppercase() == first_letter
            }
        }
    }

    pub fn into_minimized_formatter(&self) -> proc_macro2::TokenStream {
        pub enum Chunk {
            Text(String),
            Value(syn::Block),
        }

        fn into_chunks(element: &Element) -> Vec<Chunk> {
            let mut buffer = String::new();
            let mut chunks = Vec::<Chunk>::new();

            let tag_name = element.name.get_ident().map(|ident| ident.to_string()); //.expect("valid tag name");

            if let Some(tag_name) = tag_name.as_deref() {
                buffer.push_str("<");
                buffer.push_str(tag_name);

                // todo: ignore, fail, or otherwise handle attrs on empty tag better

                for attribute in &element.attributes.attributes {
                    //let key = attribute.ident();

                    match attribute {
                        ElementAttribute::Punned(key) => {
                            let attr_name = key
                                .clone()
                                .into_iter()
                                .map(|ident| format!("{}", ident))
                                .collect::<String>();

                            buffer.push_str(" ");
                            buffer.push_str(attr_name.as_str());
                        }
                        ElementAttribute::WithValue(key, block) => {
                            let attr_name = key
                                .clone()
                                .into_iter()
                                .map(|ident| format!("{}", ident))
                                .collect::<String>();

                            buffer.push_str(" ");
                            buffer.push_str(attr_name.as_str());
                            buffer.push_str("=\"");

                            match block.stmts.as_slice() {
                                [syn::Stmt::Expr(syn::Expr::Lit(syn::ExprLit { lit, .. }), None)] => {
                                    match lit {
                                        syn::Lit::Str(x) => {
                                            buffer.push_str(&format!("{}", x.value()))
                                        }
                                        syn::Lit::ByteStr(x) => buffer.push_str(&format!(
                                            "{}",
                                            String::from_utf8(x.value()).expect("valid bytestr")
                                        )),
                                        syn::Lit::Byte(x) => {
                                            buffer.push_str(&format!("{}", x.value()))
                                        }
                                        syn::Lit::Char(x) => {
                                            buffer.push_str(&format!("{}", x.value()))
                                        }
                                        syn::Lit::Int(x) => {
                                            buffer.push_str(&format!("{}", x.base10_digits()))
                                        }
                                        syn::Lit::Float(x) => {
                                            buffer.push_str(&format!("{}", x.base10_digits()))
                                        }
                                        syn::Lit::Bool(x) => {
                                            buffer.push_str(&format!("{}", x.value()))
                                        }
                                        syn::Lit::Verbatim(_) => {}
                                        _ => {}
                                    }
                                }

                                _ => {
                                    // flush buffer as text block, next is expr
                                    // todo: necessary? if buffer.len() > 0 {
                                    chunks.push(Chunk::Text(buffer.drain(..).collect()));
                                    // }
                                    chunks.push(Chunk::Value(block.clone()));
                                }
                            }

                            buffer.push_str("\"");
                        }
                    }
                }
            }

            if element.is_self_closing {
                if tag_name.is_some() {
                    // todo: are there really any html elements that should have the closing '/'?
                    buffer.push_str(" />");
                }
            } else {
                if tag_name.is_some() {
                    buffer.push_str(">");
                }

                for child in &element.children.nodes {
                    match child {
                        crate::child::Child::Element(element) => {
                            let mut child_chunks = into_chunks(element).into_iter();

                            match child_chunks.next() {
                                Some(Chunk::Text(text)) => {
                                    buffer.push_str(&text);
                                    chunks.push(Chunk::Text(buffer.drain(..).collect()));
                                }
                                Some(Chunk::Value(value)) => {
                                    if buffer.len() > 0 {
                                        chunks.push(Chunk::Text(buffer.drain(..).collect()));
                                    }
                                    chunks.push(Chunk::Value(value));
                                }
                                None => {}
                            }

                            chunks.extend(child_chunks);
                        }

                        crate::child::Child::RawBlock(block) => match block.stmts.as_slice() {
                            [syn::Stmt::Expr(syn::Expr::Lit(syn::ExprLit { lit, .. }), None)] => {
                                match lit {
                                    syn::Lit::Str(x) => buffer.push_str(&format!("{}", x.value())),
                                    syn::Lit::ByteStr(x) => buffer.push_str(&format!(
                                        "{}",
                                        String::from_utf8(x.value()).expect("valid bytestr")
                                    )),
                                    syn::Lit::Byte(x) => buffer.push_str(&format!("{}", x.value())),
                                    syn::Lit::Char(x) => buffer.push_str(&format!("{}", x.value())),
                                    syn::Lit::Int(x) => {
                                        buffer.push_str(&format!("{}", x.base10_digits()))
                                    }
                                    syn::Lit::Float(x) => {
                                        buffer.push_str(&format!("{}", x.base10_digits()))
                                    }
                                    syn::Lit::Bool(x) => buffer.push_str(&format!("{}", x.value())),
                                    syn::Lit::Verbatim(_) => {}
                                    _ => todo!(),
                                }
                            }

                            // note: this was just to experiment
                            [syn::Stmt::Expr(
                                syn::Expr::Tuple(syn::ExprTuple { elems, .. }),
                                None,
                            )] => {
                                //
                                // let elements = elems
                                //     .clone()
                                //     .into_iter()
                                //     .map(|expr| format!("{}", expr))
                                //     .collect::<String>();

                                panic!(":: {}", elems.len());
                            }

                            _ => {
                                if buffer.len() > 0 {
                                    chunks.push(Chunk::Text(buffer.drain(..).collect()));
                                }

                                //panic!("{}", block.stmts[0]);

                                chunks.push(Chunk::Value(block.clone()))
                            }
                        },
                    }
                }
                // children stuff

                if let Some(tag_name) = tag_name.as_deref() {
                    buffer.push_str("</");
                    buffer.push_str(tag_name);
                    buffer.push_str(">");
                }
            }

            chunks.push(Chunk::Text(buffer));
            chunks
        }

        let chunks = into_chunks(self);
        let chunks_len = chunks.len();

        let chunk_strs = chunks
            .iter()
            .map(|chunk| match chunk {
                Chunk::Text(text) => quote! { #text, },
                Chunk::Value(value) => quote! { #value, },
            })
            .collect::<proc_macro2::TokenStream>();

        let expr = (0..chunks_len)
            .into_iter()
            .map(|_| "{}")
            .collect::<String>();

        quote! { format_args!( #expr, #chunk_strs ) }
    }
}

impl ToTokens for Element {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;

        let declaration = if self.is_custom_element() {
            let attrs = self.attributes.for_custom_element(&self.children);
            quote! { #name #attrs }
        } else {
            let attrs = self.attributes.for_simple_element();
            let children_tuple = self.children.as_option_of_tuples_tokens();
            quote! {
                ::html_codegen::SimpleElement {
                    tag_name: stringify!(#name),
                    attributes: #attrs,
                    contents: #children_tuple,
                }
            }
        };

        declaration.to_tokens(tokens);
    }
}
