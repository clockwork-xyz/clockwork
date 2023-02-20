use super::phantom::{solana, ConnectResponse};
use wasm_bindgen_futures::spawn_local;
use yew::{function_component, html, Callback, Html, MouseEvent, Properties,  use_state};
use std::ops::Deref;
use log;

use super::button::Button;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

#[function_component]
pub fn PhantomConnect(_: &Props) -> Html {
    let account_handle = use_state(|| "".to_string());
    let account = account_handle.deref().clone();
    let has_account = account.trim().is_empty();

    let handle_click = Callback::from(move |_: MouseEvent| {
        let account_handle = account_handle.clone();
        let account = account_handle.deref().clone();

        spawn_local(async move {        
            match account != "".to_string() {
                true => {
                    let response = solana.disconnect().await;
                    log::info!("disconnected: {:?}", response);
                    account_handle.set("".to_string());
            
                }
                _ => {
                    let response = solana.connect().await;
                    log::info!("connected: {:?}", solana.is_connected());
                    if solana.is_connected() {
                        let response: ConnectResponse = response.into_serde().unwrap();
                        log::info!("disconnected: {:?}", response.public_key);
                        account_handle.set(response.public_key)
                    }
                }
            };   
        });        
    });

    let connect_hint_text =  match has_account {
        true => "Connect to Phantom Wallet".to_owned(),
        false => format!("Connected to {:?}", account)
    };

    let connect_text =  match has_account {
        true => "Login Phantom",
        false => "Logout Phantom"
    };

    html! {
        <>
            <h1  class="p-10 text-xl font-bold">
                { connect_hint_text }
            </h1>
            <Button 
                value={ connect_text }
                onclick={handle_click}
            />            
        </>
    }
}
