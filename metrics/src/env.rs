use std::{env, fmt};

#[derive(Debug)]
pub enum Envvar {
    Keypath,
    RpcEndpoint,
    WsEndpoint,
    EsCloudId,
    EsUser,
    EsPassword,
    EsIndex,
}

impl fmt::Display for Envvar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Envvar::Keypath => write!(f, "KEYPATH"),
            Envvar::RpcEndpoint => write!(f, "RPC_ENDPOINT"),
            Envvar::WsEndpoint => write!(f, "WS_ENDPOINT"),
            Envvar::EsCloudId => write!(f, "ES_CLOUD_ID"),
            Envvar::EsUser => write!(f, "ES_USER"),
            Envvar::EsPassword => write!(f, "ES_PASSWORD"),
            Envvar::EsIndex => write!(f, "ES_INDEX"),
        }
    }
}

impl Envvar {
    pub fn get(self) -> String {
        println!("Getting env {:#?}", self.to_string());
        env::var(self.to_string()).unwrap()
    }
}
