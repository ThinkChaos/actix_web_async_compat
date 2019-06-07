#![feature(box_patterns)]

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::{Ident, TokenTree};
use quote::{quote, ToTokens};
use syn::{
    parse_macro_input, ReturnType, Type,
};

fn guess_return_type(decl: &Box<syn::FnDecl>) -> Ident {
    let output = &decl.output;

    match output {
        ReturnType::Default => {
            panic!("return type should be Result");
        }
        ReturnType::Type(_, t) => match t {
            box Type::Path(p) => {
                let mut tokens = p.path.segments.clone().into_token_stream().into_iter();

                if let Some(token_tree) = tokens.next() {
                    if let TokenTree::Ident(i) = token_tree {
                        if i != "Result" {
                            panic!("should be Result ident");
                        }
                    } else {
                        panic!("should be ident");
                    }
                };

                tokens.next(); // skip "<"

                if let Some(token_tree) = tokens.next() {
                    if let TokenTree::Ident(return_type) = token_tree {
                        return_type
                    } else {
                        panic!("should be ident");
                    }
                } else {
                    panic!("must be one more token");
                }
            }
            _ => {
                panic!("NOT path returned!");
            }
        },
    }
}

#[proc_macro_attribute]
pub fn async_compat(_: TokenStream, handler: TokenStream) -> TokenStream {
    let handler = parse_macro_input!(handler as syn::ItemFn);

    let handler_name = &handler.ident;

    let handler_decl = &handler.decl;
    let handler_block = &handler.block;
    let handler_inputs = &handler_decl.inputs;
    let return_type = guess_return_type(handler_decl);

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        fn #handler_name(#handler_inputs) -> impl Future<Item = #return_type, Error = Error> {
            async move #handler_block .boxed().compat()
        }
    };

    // debug:
    //println!("{}", expanded);

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
