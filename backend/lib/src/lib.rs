extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Paws)]
pub fn paws_macro_derive(input: TokenStream) -> TokenStream {
    // Создаю представление синтаксического дерева кода Rust
    let ast = syn::parse(input).unwrap();

    impl_paws_macro(&ast)
}

fn impl_paws_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Paws for #name { }
    };
    
    gen.into()
}
