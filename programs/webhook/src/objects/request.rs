use super::Api;

use {
    crate::errors::ClockworkError,
    anchor_lang::{prelude::*, AnchorDeserialize},
    std::{
        collections::HashMap,
        convert::TryFrom,
        fmt::{Display, Formatter},
        str::FromStr,
    },
};

pub const SEED_REQUEST: &[u8] = b"request";

/**
 * Request
 */

#[account]
#[derive(Debug)]
pub struct Request {
    pub api: Pubkey,
    pub caller: Pubkey,
    pub created_at: u64,
    pub fee_amount: u64,
    pub headers: HashMap<String, String>,
    pub id: String,
    pub method: HttpMethod,
    pub route: String,
    pub url: String,
    pub workers: Vec<Pubkey>,
}

impl Request {
    pub fn pubkey(api: Pubkey, caller: Pubkey, id: String) -> Pubkey {
        Pubkey::find_program_address(
            &[SEED_REQUEST, api.as_ref(), caller.as_ref(), id.as_bytes()],
            &crate::ID,
        )
        .0
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
    fn pubkey(&self) -> Pubkey;

    fn init(
        &mut self,
        api: &Account<Api>,
        caller: Pubkey,
        created_at: u64,
        fee_amount: u64,
        headers: HashMap<String, String>,
        id: String,
        method: HttpMethod,
        route: String,
        workers: Vec<Pubkey>,
    ) -> Result<()>;
}

impl RequestAccount for Account<'_, Request> {
    fn pubkey(&self) -> Pubkey {
        Request::pubkey(self.api, self.caller, self.id.clone())
    }

    fn init(
        &mut self,
        api: &Account<Api>,
        caller: Pubkey,
        created_at: u64,
        fee_amount: u64,
        headers: HashMap<String, String>,
        id: String,
        method: HttpMethod,
        route: String,
        workers: Vec<Pubkey>,
    ) -> Result<()> {
        self.api = api.key();
        self.caller = caller;
        self.created_at = created_at;
        self.fee_amount = fee_amount;
        self.headers = headers;
        self.id = id;
        self.method = method;
        self.route = route.clone();
        self.url = api.clone().base_url.to_owned() + route.as_str();
        self.workers = workers;
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
