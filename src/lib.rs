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

    let mut tokens = match output {
        ReturnType::Type(_, box Type::Path(p)) => p.path.segments.clone().into_token_stream().into_iter(),
        _ => panic!("expected return type to be actix_web::Result<_>"),
    };

    // actix_web::Result / Result
    if let Some(TokenTree::Ident(mut i)) = tokens.next() {
        if i == "actix_web" {
            for _ in 0..2 {
                if let Some(TokenTree::Punct(p)) = tokens.next() {
                    if p.as_char() != ':' {
                        panic!("expected return type to be 'actix_web::Result<_>'");
                    }
                }
                else {
                    panic!("expected return type to be 'actix_web::Result<_>'");
                }
            }

            match tokens.next() {
                Some(TokenTree::Ident(i_)) => i = i_,
                _ => panic!("expected return type to be 'actix_web::Result<_>'"),
            }
        }

        if i != "Result" {
            panic!("expected return type to be 'actix_web::Result<_>'");
        }
    } else {
        panic!("expected return type identifier");
    }

    // <
    if let Some(TokenTree::Punct(p)) = tokens.next() {
        if p.as_char() != '<' {
            panic!("expected '<' token after 'Result'");
        }
    }
    else {
        panic!("expected '<' token after 'Result'");
    }

    // Finally, the type inside Result<_>
    if let Some(token_tree) = tokens.next() {
        if let TokenTree::Ident(return_type) = token_tree {
            return_type
        } else {
            panic!("expected an identifier after 'Result<'");
        }
    } else {
        panic!("expected actix_web::Result to be generic on return type");
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

    // Build the output, using quasi-quotation
    let expanded = quote! {
        fn #handler_name(#handler_inputs) -> impl ::futures::Future<Item = #return_type, Error = ::actix_web::Error> {
            async move #handler_block .boxed().compat()
        }
    };

    // debug:
    //println!("{}", expanded);

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
