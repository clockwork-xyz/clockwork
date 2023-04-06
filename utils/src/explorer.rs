const EXPLORER_URL: &str = "https://explorer.solana.com";
const CK_EXPLORER_URL: &str = "https://explorer.clockwork.xyz";

#[derive(Default)]
pub struct Explorer {
    cluster: String,
    custom_rpc: Option<String>,
}

impl From<String> for Explorer {
    fn from(json_rpc_url: String) -> Self {
        match &json_rpc_url.to_lowercase() {
            url if url.contains("devnet") => Explorer::devnet(),
            url if url.contains("testnet") => Explorer::testnet(),
            url if url.contains("mainnet") => Explorer::mainnet(),
            _ => Explorer::custom(json_rpc_url),
        }
    }
}

impl Explorer {
    pub fn mainnet() -> Self {
        Self {
            cluster: "mainnet-beta".into(),
            ..Default::default()
        }
    }

    pub fn testnet() -> Self {
        Self {
            cluster: "testnet".into(),
            ..Default::default()
        }
    }

    pub fn devnet() -> Self {
        Self {
            cluster: "devnet".into(),
            ..Default::default()
        }
    }

    pub fn custom(custom_rpc: String) -> Self {
        Self {
            cluster: "custom".into(),
            custom_rpc: Some(custom_rpc),
        }
    }

    /// Ex: https://explorer.solana.com/tx/{tx}
    ///     ?cluster=custom
    ///     &customUrl=http://localhost:8899
    pub fn tx_url<T: std::fmt::Display>(&self, tx: T) -> String {
        let url = format!("{}/tx/{}?cluster={}", EXPLORER_URL, tx, self.cluster);
        if self.cluster == "custom" {
            url + "&customUrl=" + self.custom_rpc.as_ref().unwrap()
        } else {
            url
        }
    }

    /// Ex: https://explorer.clockwork.xyz/thread/{thread}
    ///     ?network=custom
    ///     &customRPC=http://localhost:8899
    pub fn thread_url<T: std::fmt::Display, U: std::fmt::Display>(
        &self,
        thread: T,
        program_id: U,
    ) -> String {
        let url = format!(
            "{}/address/{}?programID={}&network={}",
            CK_EXPLORER_URL, thread, program_id, self.cluster
        );
        if self.cluster == "custom" {
            url + "&customRPC=" + self.custom_rpc.as_ref().unwrap()
        } else {
            url
        }
    }
}
