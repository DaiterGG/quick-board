extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::parse_macro_input;
use syn::{Ident, Token, braced, token};

struct Node {
    variant: Ident,
    element_type: Ident,
    children: Vec<Node>,
}
impl Parse for Node {
    fn parse(input: ParseStream) -> Result<Self> {
        let variant: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let element_type: Ident = input.parse()?;

        // Check if there's a brace for children
        let children = if input.peek(token::Brace) {
            let content;
            braced!(content in input);
            content
                .parse_terminated(Node::parse, Token![,])?
                .into_iter()
                .collect()
        } else {
            Vec::new()
        };
        Ok(Node {
            variant,
            element_type,
            children,
        })
    }
}

struct MacroInput {
    nodes: Vec<Node>,
}

impl Parse for MacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        braced!(content in input);
        let nodes = content.parse_terminated(Node::parse, Token![,])?;
        Ok(MacroInput {
            nodes: nodes.into_iter().collect(),
        })
    }
}

fn flatten_nodes<'a>(nodes: &'a [Node], result: &mut Vec<&'a Node>) {
    for node in nodes {
        result.push(node);
        flatten_nodes(&node.children, result);
    }
}

#[proc_macro]
pub fn markup(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as MacroInput);
    let mut all_nodes = Vec::new();
    flatten_nodes(&input.nodes, &mut all_nodes);

    let variants = all_nodes.iter().map(|n| &n.variant);
    let elements = all_nodes.iter().map(|node| {
        let variant = &node.variant;
        let element_type = &node.element_type;
        let children = node.children.iter().map(|child| &child.variant);
        quote! {
            Some(UIElement::new(
                ElementType::#element_type,
                Id::#variant.into(),
                vec![#(Id::#children.into()),*]
            ))
        }
    });

    let expanded = quote! {
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
        #[allow(non_camel_case_types)]
        pub enum Id {
            #(#variants,)*
            Total,
        }

        pub struct Predefined;

        impl Predefined {
            pub fn init() -> Vec<Option<UIElement>> {
                vec![
                    #(#elements,)*
                ]
            }
        }
    };

    TokenStream::from(expanded)
}
