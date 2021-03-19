use proc_macro::{TokenStream};

#[proc_macro_derive(Cms)]
pub fn cms_macro_derive(input: TokenStream) -> TokenStream {
    eprintln!("{:?}", input);
    TokenStream::new()
}

fn impl_crud() -> TokenStream {
    TokenStream::new()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
