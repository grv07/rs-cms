use quote::quote;
use proc_macro2;
use proc_macro::{TokenStream};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Cms)]
pub fn cms_macro_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let output = impl_crud(&input);
    proc_macro::TokenStream::from(output)
}

fn impl_crud(derive_in: &DeriveInput) -> proc_macro2::TokenStream {
    eprintln!("{:#?}", derive_in);
    let ident = &derive_in.ident; 
    quote! {
            impl Base for #ident {
            fn create() {}
            }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
