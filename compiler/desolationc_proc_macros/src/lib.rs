use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident, LitStr};

#[proc_macro]
pub fn stringify_lower(input: TokenStream) -> TokenStream {
    let ident = parse_macro_input!(input as Ident);
    let lowercase_ident = ident.to_string().to_lowercase();
    let result = quote! {
        #lowercase_ident
    };
    result.into()
}

#[proc_macro]
pub fn length(input: TokenStream) -> TokenStream {
    let ident = parse_macro_input!(input as Ident);
    let len = ident.to_string().len();
    let result = quote!(#len);
    result.into()
}

/// Splits a string into a tuple of chars.
#[proc_macro]
pub fn split_str(input: TokenStream) -> TokenStream {
    let string = parse_macro_input!(input as LitStr).value();
    let chars: Vec<char> = string.chars().collect();
    let result = quote!((#(#chars),*));
    result.into()
}

#[proc_macro]
pub fn split_lexeme(input: TokenStream) -> TokenStream {
    let string = parse_macro_input!(input as LitStr).value();
    assert!(matches!(string.len(), 1 | 2), "Expected a string of length 1 or 2.");
    let chars: Vec<char> = string.chars().collect();
    let char0 = chars.first().unwrap();
    let char1 = chars.get(1);
    let result;
    if let Some(char1) = char1 {
        result = quote!((#char0, Some(#char1)));
    } else {
        result = quote!((#char0, _));
    }
    result.into()
}
