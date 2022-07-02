use std::collections::HashMap;

use {
    anchor_lang::{prelude::*, AnchorDeserialize},
    std::convert::TryFrom,
};

pub const SEED_REQUEST: &[u8] = b"request";

/**
 * Request
 */

#[account]
#[derive(Debug)]
pub struct Request {
    pub headers: HashMap<String, String>,
    pub method: HttpMethod,
    pub owner: Pubkey,
    pub url: String,
}

// TODO Seeds

impl Request {
    pub fn pubkey() -> Pubkey {
        Pubkey::find_program_address(&[SEED_REQUEST], &crate::ID).0
    }
}

impl TryFrom<Vec<u8>> for Request {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        Request::try_deserialize(&mut data.as_slice())
    }
}

/**
 * RequestAccount
 */

pub trait RequestAccount {
    fn new(
        &mut self,
        headers: HashMap<String, String>,
        method: HttpMethod,
        owner: Pubkey,
        url: String,
    ) -> Result<()>;
}

impl RequestAccount for Account<'_, Request> {
    fn new(
        &mut self,
        headers: HashMap<String, String>,
        method: HttpMethod,
        owner: Pubkey,
        url: String,
    ) -> Result<()> {
        self.headers = headers;
        self.method = method;
        self.owner = owner;
        self.url = url;
        Ok(())
    }
}

/**
 * HttpMethod
 */

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Debug, PartialEq)]
pub enum HttpMethod {
    Get,
    Post,
}
