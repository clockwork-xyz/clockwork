use {
    crate::errors::CronosError,
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
    pub headers: HashMap<String, String>,
    pub method: HttpMethod,
    pub owner: Pubkey,
    pub url: String,
    // TODO Track when this request created (to be used for timeouts)
    // TODO Hold onto lamport funds for the reward
    // TODO Track who this reward can be released to
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
            _ => Err(CronosError::InvalidHttpMethod.into()),
        }
    }
}
