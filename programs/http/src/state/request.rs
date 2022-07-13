use {
    super::Manager,
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
    pub ack_authority: Pubkey,
    pub created_at: u64,
    pub fee_amount: u64,
    pub headers: HashMap<String, String>,
    pub id: u128,
    pub manager: Pubkey,
    pub method: HttpMethod,
    pub url: String,
    // TODO Hold onto lamport funds for the reward
    // TODO Track who this reward can be released to
}

impl Request {
    pub fn pubkey(manager: Pubkey, id: u128) -> Pubkey {
        Pubkey::find_program_address(
            &[
                SEED_REQUEST,
                manager.key().as_ref(),
                id.to_be_bytes().as_ref(),
            ],
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
    fn new(
        &mut self,
        ack_authority: Pubkey,
        created_at: u64,
        fee_amount: u64,
        headers: HashMap<String, String>,
        manager: &mut Account<Manager>,
        method: HttpMethod,
        url: String,
    ) -> Result<()>;
}

impl RequestAccount for Account<'_, Request> {
    fn new(
        &mut self,
        ack_authority: Pubkey,
        created_at: u64,
        fee_amount: u64,
        headers: HashMap<String, String>,
        manager: &mut Account<Manager>,
        method: HttpMethod,
        url: String,
    ) -> Result<()> {
        // Initialize the request data
        self.ack_authority = ack_authority;
        self.created_at = created_at;
        self.fee_amount = fee_amount;
        self.headers = headers;
        self.id = manager.clone().into_inner().request_count;
        self.manager = manager.key();
        self.method = method;
        self.url = url;

        // Increment the manager's request count
        manager.request_count = manager
            .clone()
            .into_inner()
            .request_count
            .checked_add(1)
            .unwrap();

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
