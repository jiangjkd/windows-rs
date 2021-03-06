use squote::{quote, TokenStream};
use std::iter::FromIterator;

pub fn gen_namespace(destination: &str, source: &str) -> TokenStream {
    if destination == source {
        return TokenStream::new();
    }

    let mut tokens = Vec::new();
    let mut source = source.split('.').peekable();
    let mut destination = destination.split('.').peekable();

    while source.peek() == destination.peek() {
        if source.next().is_none() {
            break;
        }
        destination.next();
    }

    let count = source.count();

    if count > 0 {
        tokens.resize(tokens.len() + count, quote! { super:: });
    }

    tokens.extend(destination.map(|destination| {
        let destination = crate::format_ident(&crate::to_snake(destination));
        quote! { #destination:: }
    }));

    TokenStream::from_iter(tokens)
}

pub fn gen_full_namespace(destination: &str) -> TokenStream {
    let mut tokens = TokenStream::new();

    for destination in destination.split('.') {
        let destination = crate::format_ident(&crate::to_snake(destination));

        tokens.combine(&quote! { #destination:: });
    }

    tokens
}
