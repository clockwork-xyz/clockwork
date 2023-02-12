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

// use crate::my_program::InstructionBuilder;
// fn main() {
//     InstructionBuilder::my_ix::build(
//         MyInstructionAccounts {
//             pubkey_0,
//             pubkey_1,
//             pubkey_2,
//         },
//         MyInstructionData {
//             arg_0,
//             arg_1,
//         }
//     )
// }

// let accs = crate::accounts::FeeClaim {
//     authority: Pubkey::new_rand(),
//     pay_to: Pubkey::new_rand(),
//     fee: Pubkey::new_rand(),
// }
// let datas = crate::instruction::FeeClaim {
//     amount: todo!(),
// };

// #[proc_macro_derive(Clockwork)]
// pub fn derive_clockwork(input: TokenStream) -> TokenStream {
//     let accounts_struct = parse_macro_input!(input as anchor_syn::AccountsStruct);
//     let clockwork_target: proc_macro2::TokenStream = {
//         let ident: proc_macro2::TokenStream = format!("{}Instruction", accounts_struct.ident)
//             .parse()
//             .unwrap();
//         let pubkey_args: Vec<proc_macro2::TokenStream> = accounts_struct
//             .field_names()
//             .iter()
//             .map(|f| f.parse().unwrap())
//             .collect();
//         let account_metas: Vec<proc_macro2::TokenStream> = accounts_struct
//             .fields
//             .iter()
//             .map(|f| {
//                 let (name, is_signer, is_optional) = match f {
//                     anchor_syn::AccountField::CompositeField(s) => (&s.ident, quote! {None}, false),
//                     anchor_syn::AccountField::Field(f) => {
//                         let is_signer = match f.constraints.is_signer() {
//                             false => quote! {false},
//                             true => quote! {true},
//                         };
//                         (&f.ident, is_signer, f.is_optional)
//                     }
//                 };
//                 quote! {
//                     SerializableAccount {
//                         pubkey: #name,
//                         is_signer: #is_signer,
//                         is_writable: #is_optional
//                     }
//                 }
//             })
//             .collect();
//         let data_args: Vec<proc_macro2::TokenStream> =
//             accounts_struct.instruction_args().map_or(vec![], |args| {
//                 args.iter()
//                     .map(|(k, v)| {
//                         let arg_name: proc_macro2::TokenStream = k.parse().unwrap();
//                         let arg_type: proc_macro2::TokenStream = v.parse().unwrap();
//                         quote! {
//                             #arg_name: #arg_type
//                         }
//                     })
//                     .collect()
//             });
//         let ix_data: proc_macro2::TokenStream = {
//             let ix_data_ident = accounts_struct.ident.clone();
//             let ix_data_args: Vec<proc_macro2::TokenStream> =
//                 accounts_struct.instruction_args().map_or(vec![], |args| {
//                     args.iter()
//                         .map(|(k, _)| {
//                             let arg_name: proc_macro2::TokenStream = k.parse().unwrap();
//                             quote! {#arg_name}
//                         })
//                         .collect()
//                 });
//             quote! {
//                 crate::instruction::#ix_data_ident {
//                     #(#ix_data_args),*
//                 }.data()
//             }
//         };
//         quote! {
//             pub struct #ident {}
//             impl #ident {
//                 pub fn build(
//                     #(#pubkey_args: Pubkey),*
//                     #(#data_args),*
//                 ) -> SerializableInstruction {
//                     SerializableInstruction  {
//                         program_id: crate::ID,
//                         accounts: vec![
//                             #(#account_metas),*
//                         ],
//                         data: #ix_data
//                     }
//                 }
//             }
//         }
//     };
//     quote! {
//         #clockwork_target
//     }
//     .into()
// }
