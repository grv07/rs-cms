use proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::{DeriveInput, Fields, Data};

#[proc_macro_derive(BaseCrud)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    println!("{:?}", input);
    let ast: DeriveInput = syn::parse(input).unwrap();
    match &ast.data {
      Data::Struct(ref data) => {
          println!("thisghdc ");
          for field in data.fields.iter() {
              println!("{:?}", field.ident);
          }
        },
        _ => {println!("Insise this");}
    };
    // Build the trait implementation
    impl_base_crud_macro(&ast)
}

fn impl_base_crud_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let group = &ast.data;
    //println!("{:?}", &name);
    //println!("{:?}", group);
    let gen = quote! {
        impl BaseCrud for #name {
            fn create() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}
