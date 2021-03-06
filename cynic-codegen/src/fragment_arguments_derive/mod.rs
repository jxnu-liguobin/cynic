use proc_macro2::TokenStream;

pub fn fragment_arguments_derive(ast: &syn::DeriveInput) -> Result<TokenStream, syn::Error> {
    use quote::quote;

    let ident = &ast.ident;
    Ok(quote! {
        impl ::cynic::FragmentArguments for #ident {}

        impl<'a> ::cynic::FromArguments<&'a #ident> for &'a () {
            fn from_arguments(_: &#ident) -> &() {
                &()
            }
        }
    })
}
