use convert_case::Case::{Pascal, Snake};
use convert_case::Casing;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, quote_spanned, ToTokens};
use rstml::node::{KeyedAttribute, Node, NodeAttribute, NodeElement, NodeName};
use syn::spanned::Spanned;
use syn::{parse_quote, LitStr};

pub fn render_view(nodes: &[Node]) -> Option<TokenStream> {
    match nodes.len() {
        0 => {
            let span = Span::call_site();
            Some(quote_spanned! {
                span => ()
            })
        }
        1 => node_to_tokens(&nodes[0], TagType::Root),
        _ => fragment_to_tokens(nodes, TagType::Root),
    }
}

fn element_children_to_tokens(nodes: &[Node], parent_type: TagType) -> Option<TokenStream> {
    let children = children_to_tokens(nodes, parent_type)
        .into_iter()
        .map(|child| {
            quote! {
                #child,
            }
        });
    Some(quote! {
        #(#children)*
    })
}

fn fragment_to_tokens(nodes: &[Node], _parent_type: TagType) -> Option<TokenStream> {
    let children = children_to_tokens(nodes, TagType::Fragment);
    if children.is_empty() {
        Some(quote! {
            ::mrml::fragment::Fragment::default()
        })
    } else {
        Some(quote! {
            ::mrml::fragment::Fragment::from(vec![#(#children),*])
        })
    }
}

fn children_to_tokens(nodes: &[Node], parent_type: TagType) -> Vec<TokenStream> {
    let nodes = nodes
        .iter()
        .filter_map(|node| node_to_tokens(node, parent_type))
        .collect();
    nodes
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) enum TagType {
    Root,
    Fragment,
    Mjml,
    Html,
    MjmlAttributes,
}

fn node_to_tokens(node: &Node, parent_type: TagType) -> Option<TokenStream> {
    match node {
        Node::Fragment(fragment) => fragment_to_tokens(&fragment.children, parent_type),
        Node::Block(block) => Some(quote! { #block }),
        Node::Comment(text) => Some(comment_to_tokens(&text.value)),
        Node::Text(text) => Some(text_to_tokens(&text.value)),
        Node::RawText(raw) => {
            let text = raw.to_string_best();
            let text = syn::LitStr::new(&text, raw.span());
            Some(text_to_tokens(&text))
        }
        Node::Element(node) => element_to_tokens(node, parent_type),
        _ => None,
    }
    .map(|tt| -> TokenStream {
        if parent_type == TagType::Root {
            return tt;
        }
        match node {
            Node::Block(_) => tt,
            _ => quote! {#tt.into()},
        }
    })
}

fn text_to_tokens(text: &LitStr) -> TokenStream {
    quote! { mrml::text::Text::from(#text) }
}
fn comment_to_tokens(text: &LitStr) -> TokenStream {
    quote! { mrml::comment::Comment::from(#text) }
}

pub(crate) fn element_to_tokens(node: &NodeElement, parent_type: TagType) -> Option<TokenStream> {
    let tag_type: TagType;
    let name = node.name();
    let tag = name.to_string();

    let name = if parent_type == TagType::MjmlAttributes {
        tag_type = TagType::Mjml;
        if tag == "mj-all" {
            quote! { ::mrml::mj_attributes_all::MjAttributesAll::default() }
        } else if tag == "mj-class" {
            quote! { ::mrml::mj_attributes_class::MjAttributesClass::default() }
        } else {
            quote! { ::mrml::mj_attributes_element::MjAttributesElement::new(#tag.to_string()) }
        }
    } else if is_mjml_element(&tag) {
        if tag == "mj-attributes" {
            tag_type = TagType::MjmlAttributes
        } else {
            tag_type = TagType::Mjml
        }
        let snake = Ident::new(&convert_to_snake_case(tag.clone()), name.span());
        let pascal = Ident::new(&convert_to_pascal_case(tag.clone()), name.span());
        if is_mjml_text_element(&tag) {
            let val = node
                .children
                .iter()
                .map(|c| match c {
                    Node::Text(t) => t.value_string(),
                    Node::RawText(t) => t.to_string_best(),
                    _ => proc_macro_error::abort!(
                        node.span(),
                        "Non-text nodes are not supported as children of text nodes"
                    ),
                })
                .fold(String::new(), |a, b| a + &b + "");

            quote! { ::mrml::#snake::#pascal::from(#val) }
        } else {
            quote! { ::mrml::#snake::#pascal::default() }
        }
    } else {
        tag_type = TagType::Html;
        quote! { ::mrml::node::Node::from(#tag) }
    };

    let attributes = node.attributes();
    let attributes = if attributes.len() == 1 {
        Some(attribute_to_tokens(&tag, &attributes[0], tag_type))
    } else {
        let nodes = attributes
            .iter()
            .map(|node| attribute_to_tokens(&tag, node, tag_type));
        Some(quote! {
            #(#nodes)*
        })
    };

    let self_closing = is_self_closing(node);
    let children = if !self_closing && !is_mjml_text_element(&tag) {
        element_children_to_tokens(&node.children, tag_type)
    } else {
        if !is_mjml_text_element(&tag) && !node.children.is_empty() {
            let name = node.name();
            proc_macro_error::emit_error!(
                name.span(),
                format!(
                    "Self-closing elements like <{name}> cannot have \
                         children."
                )
            );
        };
        None
    };

    if let Some(children) = children.filter(|c| !c.is_empty()) {
        Some(quote! {
            ::mrmx::WithChildren::with_children(
                #name
                #attributes,
                vec![#children]
            )
        })
    } else {
        Some(quote! {
            #name
            #attributes
        })
    }
}

fn attribute_to_tokens(tag_name: &str, node: &NodeAttribute, tag_type: TagType) -> TokenStream {
    match node {
        NodeAttribute::Block(node) => {
            proc_macro_error::abort!(
                node.span(),
                "Code blocks in attributes are not yet supported"
            )
        }
        NodeAttribute::Attribute(node) => {
            if tag_type == TagType::Html || !is_mjml_not_free_attributes(tag_name) {
                let key = &node.key.to_string();
                let none = parse_quote! { "" };
                let value = &node.value().unwrap_or(&none);
                quote! {
                    .with_attribute(#key.to_string(), #value.to_string())
                }
            } else {
                let key = attribute_name(&node.key);
                let value = attribute_value(node);
                quote! {
                    .#key(#value)
                }
            }
        }
    }
}

fn is_self_closing(node: &NodeElement) -> bool {
    // self-closing tags
    // https://developer.mozilla.org/en-US/docs/Glossary/Empty_element
    // Keep list alphabetized for binary search
    [
        "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param",
        "source", "track", "wbr",
    ]
    .binary_search(&node.name().to_string().as_str())
    .is_ok()
}

pub(crate) fn is_mjml_element(tag: &str) -> bool {
    // Keep list alphabetized for binary search
    [
        "mj-accordion",
        "mj-accordion-element",
        "mj-accordion-text",
        "mj-accordion-title",
        "mj-attributes",
        "mj-body",
        "mj-breakpoint",
        "mj-button",
        "mj-carousel",
        "mj-carousel-image",
        "mj-column",
        "mj-divider",
        "mj-font",
        "mj-group",
        "mj-head",
        "mj-hero",
        "mj-image",
        "mj-navbar",
        "mj-navbar-link",
        "mj-preview",
        "mj-raw",
        "mj-section",
        "mj-social",
        "mj-social-element",
        "mj-spacer",
        "mj-style",
        "mj-table",
        "mj-text",
        "mj-title",
        "mj-wrapper",
        "mjml",
    ]
    .binary_search(&tag)
    .is_ok()
}

fn is_mjml_text_element(tag: &str) -> bool {
    // Keep list alphabetized for binary search
    ["mj-preview", "mj-style", "mj-title", "mj_preview"]
        .binary_search(&tag)
        .is_ok()
}
fn is_mjml_not_free_attributes(tag: &str) -> bool {
    // Keep list alphabetized for binary search
    [
        // "mj-breakpoint",
        "mj-head",
        "mj-include",
        "mj-include-body",
        "mj-include-head",
        "mj-preview",
        "mj-raw",
        "mj-title",
    ]
    .binary_search(&tag)
    .is_ok()
}

fn attribute_name(name: &NodeName) -> TokenStream {
    let s = name.to_string();
    Ident::new_raw(&s.replace('-', "_"), name.span()).to_token_stream()
}

fn attribute_value(attr: &KeyedAttribute) -> TokenStream {
    match attr.value() {
        Some(value) => {
            quote! { #value }
        }
        None => quote! { true },
    }
}

pub(crate) fn convert_to_snake_case(name: String) -> String {
    if !name.is_case(Snake) {
        name.to_case(Snake)
    } else {
        name
    }
}

pub(crate) fn convert_to_pascal_case(name: String) -> String {
    if !name.is_case(Pascal) {
        name.to_case(Pascal)
    } else {
        name
    }
}
