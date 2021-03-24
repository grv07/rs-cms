use proc_macro::TokenStream;
use proc_macro2;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Cms)]
pub fn cms_macro_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let output = impl_crud(&input);
    proc_macro::TokenStream::from(output)
}

fn get_fields(input: &DeriveInput) -> &syn::punctuated::Punctuated<syn::Field, syn::token::Comma> {
    let fields = match &input.data {
        // TODO: Handle other cases.
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
            ..
        }) => Some(named),
        _ => None,
    };
    // TODO: Handle this error case.
    fields.unwrap()
}

fn impl_crud(input: &DeriveInput) -> proc_macro2::TokenStream {
    eprintln!("{:#?}", input);
    let ident = &input.ident;
    let fields = get_fields(input);
    eprintln!("{:#?}", fields);
    quote! {
            struct Test {
                #fields
            }
            impl #ident {
                fn insert<T>(conn: &T, #fields) {}
            }
            impl Base for #ident {
                fn update() {}
                fn delete() {}
                fn read() {}
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
