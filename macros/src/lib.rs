extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
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

// /// The `#[program]` attribute defines the module containing all instruction
// /// handlers defining all entries into a Solana program.
// #[proc_macro_attribute]
// pub fn program(
//     _args: proc_macro::TokenStream,
//     input: proc_macro::TokenStream,
// ) -> proc_macro::TokenStream {
//     parse_macro_input!(input as anchor_syn::Program)
//         .to_token_stream()
//         .into()
// }

// #[proc_macro_derive(Clockwork)]
#[proc_macro_attribute]
pub fn clockwork(_args: TokenStream, input: TokenStream) -> TokenStream {
    let program = parse_macro_input!(input as anchor_syn::Program);
    let clockwork_targets: Vec<proc_macro2::TokenStream> = program
        .ixs
        .iter()
        .map(|ix| {
            let name = format!("Clockwork{}", ix.ident);
            quote! {
                type #name = u64;
            }
        })
        .collect();
    quote! {
        #(#clockwork_targets)*
        todo!()
    }
    .into()
}
