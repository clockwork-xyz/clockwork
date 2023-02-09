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

#[proc_macro_attribute]
pub fn clockwork(_args: TokenStream, input: TokenStream) -> TokenStream {
    let program = parse_macro_input!(input as anchor_syn::Program);
    let clockwork_targets: Vec<proc_macro2::TokenStream> = program
        .ixs
        .iter()
        .map(|ix| {
            ix
            // let name: proc_macro2::TokenStream =
            //     format!("Clockwork{}", ix.anchor_ident).parse().unwrap();
            // TODO Create a cleaner instruction building interface.
            quote! {
                // type #name = u64;
            }
        })
        .collect();
    quote! {
        #(#clockwork_targets)*
    }
    .into()
}
