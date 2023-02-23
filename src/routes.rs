use anchor_lang::prelude::Pubkey;

#[derive(Clone, PartialEq)]
pub enum RoutePath {
    Home,
    Data,
    Files,
    PriceFeed { address: Pubkey },
    Threads,
    NotFound,
}

impl RoutePath {
    pub fn to_string(&self) -> String {
        match self {
            RoutePath::Home => "/".to_owned(),
            RoutePath::Data => "/data".to_owned(),
            RoutePath::PriceFeed { address } => format!("/price_feed/{:?}", address),
            RoutePath::Files => "/files".to_owned(),
            RoutePath::Threads => "/threads".to_owned(),
            RoutePath::NotFound => "".to_owned(),
        }
    }

    pub fn generic_path<'a>(&self) -> &'a str {
        match self {
            RoutePath::Home => "/",
            RoutePath::Data => "/data",
            RoutePath::PriceFeed { address: _ } => "/price_feed/:address",
            RoutePath::Files => "/files",
            RoutePath::Threads => "/threads",
            RoutePath::NotFound => "",
        }
    }
}
