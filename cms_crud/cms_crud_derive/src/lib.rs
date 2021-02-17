use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(BaseCrud)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    println!("{:?}", input);
    let ast = syn::parse(input).unwrap();
    // Build the trait implementation
    impl_base_crud_macro(&ast)
}

fn impl_base_crud_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let group = &ast.attrs;
    println!("{:?}", &name);
    println!("{:?}", group);
    let gen = quote! {
        impl BaseCrud for #name {
            fn create() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}
