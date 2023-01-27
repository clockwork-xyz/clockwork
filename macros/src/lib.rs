extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_derive(TryFromData)]
pub fn derive_try_from_data_attr(input: TokenStream) -> TokenStream {
    let account_struct = parse_macro_input!(input as syn::ItemStruct);
    let account_name = &account_struct.ident;
    let (impl_gen, ty_gen, where_clause) = account_struct.generics.split_for_impl();
    proc_macro::TokenStream::from(quote! {
        #[automatically_derived]
        impl #impl_gen TryFrom<Vec<u8>> for #account_name #ty_gen #where_clause {
            type Error = Error;
            fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
                #account_name::try_deserialize(&mut data.as_slice())
            }
        }
    })
}
