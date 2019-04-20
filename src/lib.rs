#![feature(box_patterns)]

extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenTree};
use quote::{quote, ToTokens};
use syn::{
    parse_macro_input,
    punctuated::{Pair, Punctuated},
    token::Comma,
    FnArg, ReturnType, Type,
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

fn guess_fn_args(inputs: &Punctuated<FnArg, Comma>) -> proc_macro2::TokenStream {
    let mut stream = proc_macro2::TokenStream::new();

    for pair in inputs.pairs() {
        match pair {
            Pair::Punctuated(_fn_arg, _comma) => {
                unimplemented!();
            }
            Pair::End(fn_arg) => {
                match fn_arg {
                    FnArg::Captured(arg) => {
                        let pat = &arg.pat;
                        pat.to_owned().to_tokens(&mut stream);
                        // println!("ARG: {:?}", quote!(#x));
                    }
                    _ => {
                        unimplemented!();
                    }
                }
            }
        }
    }

    stream
}

#[proc_macro_attribute]
pub fn async_compat(_: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::ItemFn);

    let fn_name = &input.ident;
    let backup_fn_name = format!("__async_{}", fn_name.to_string());
    let backup_fn_name = Ident::new(&backup_fn_name, Span::call_site());

    let original_fn_decl = &input.decl;
    let original_fn_return_type = &original_fn_decl.output;
    let original_fn_block = &input.block;
    let original_fn_inputs = &original_fn_decl.inputs;
    let return_type = guess_return_type(original_fn_decl);

    let fn_args = guess_fn_args(original_fn_inputs);

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        fn #fn_name(#original_fn_inputs) -> impl Future<Item = #return_type, Error = Error> {
            tokio_async_await::compat::backward::Compat::new(#backup_fn_name(#fn_args))
        }

        async fn #backup_fn_name(#original_fn_inputs) #original_fn_return_type #original_fn_block
    };

    // debug:
    // println!("{}", expanded);

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
