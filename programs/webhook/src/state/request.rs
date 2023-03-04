use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
    str::FromStr,
};

use anchor_lang::{prelude::*, AnchorDeserialize};
use serde::{Deserialize, Serialize};

use crate::errors::ClockworkError;

pub const SEED_REQUEST: &[u8] = b"request";

#[account]
#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    pub authority: Pubkey,
    pub body: Vec<u8>,
    pub created_at: u64,
    pub headers: HashMap<String, String>,
    pub id: Vec<u8>,
    pub method: HttpMethod,
    pub relayer: Relayer,
    pub url: String,
    pub workers: Vec<Pubkey>,
}

impl Request {
    pub fn pubkey(authority: Pubkey, id: Vec<u8>) -> Pubkey {
        Pubkey::find_program_address(
            &[SEED_REQUEST, authority.as_ref(), id.as_slice()],
            &crate::ID,
        )
        .0
    }
}

/// RequestAccount ...
pub trait RequestAccount {
    fn pubkey(&self) -> Pubkey;
}

impl RequestAccount for Account<'_, Request> {
    fn pubkey(&self) -> Pubkey {
        Request::pubkey(self.authority, self.id.clone())
    }
}

/// HttpMethod
#[derive(AnchorDeserialize, AnchorSerialize, Deserialize, Serialize, Clone, Debug, PartialEq)]
pub enum HttpMethod {
    Get,
    Post,
}

impl Display for HttpMethod {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match *self {
            HttpMethod::Get => write!(f, "GET"),
            HttpMethod::Post => write!(f, "POST"),
        }
    }
}

impl FromStr for HttpMethod {
    type Err = Error;

    fn from_str(input: &str) -> std::result::Result<HttpMethod, Self::Err> {
        match input.to_uppercase().as_str() {
            "GET" => Ok(HttpMethod::Get),
            "POST" => Ok(HttpMethod::Post),
            _ => Err(ClockworkError::InvalidHttpMethod.into()),
        }
    }
}

#[derive(AnchorDeserialize, AnchorSerialize, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum Relayer {
    Clockwork,
    Custom(String),
}
