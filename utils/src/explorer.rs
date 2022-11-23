const EXPLORER_URL: &str = "https://explorer.solana.com";
const CK_EXPLORER_URL: &str = "https://explorer.clockwork.xyz";

pub struct Explorer {
    cluster: &'static str,
    custom_rpc: &'static str,
}

impl Explorer {
    pub const fn mainnet() -> Self {
        Self {
            cluster: "mainnet-beta",
            custom_rpc: "",
        }
    }

    pub const fn testnet() -> Self {
        Self {
            cluster: "testnet",
            custom_rpc: "",
        }
    }

    pub const fn devnet() -> Self {
        Self {
            cluster: "devnet",
            custom_rpc: "",
        }
    }

    pub const fn custom() -> Self {
        Self {
            cluster: "custom",
            custom_rpc: "http://localhost:8899",
        }
    }

    /// Ex: https://explorer.solana.com/tx/{tx}
    ///     ?cluster=custom
    ///     &customUrl=http://localhost:8899
    pub fn tx_url<T: std::fmt::Display>(&self, tx: T) -> String {
        let url = format!("{}/tx/{}?cluster={}", EXPLORER_URL, tx, self.cluster);
        if self.cluster == "custom" {
            url + "&customUrl=" + self.custom_rpc
        } else {
            url
        }
    }

    /// Ex: https://explorer.clockwork.xyz/thread/{thread}
    ///     ?network=custom
    ///     &customRPC=http://localhost:8899
    pub fn thread_url<T: std::fmt::Display, U: std::fmt::Display>(&self, thread: T, program_id: U) -> String {
        let url = format!("{}/thread/{}?network={}&programID={}", CK_EXPLORER_URL, thread, self
            .cluster, program_id);
        if self.cluster == "custom" {
            url + "&customRPC=" + self.custom_rpc
        } else {
            url
        }
    }
}
