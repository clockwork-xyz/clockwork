// Copyright 2022 Blockdaemon Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use {
    serde::Deserialize,
    solana_accountsdb_plugin_interface::accountsdb_plugin_interface::{
        AccountsDbPluginError, Result as PluginResult,
    },
    std::{fs::File, path::Path},
};

/// Plugin config.
#[derive(Deserialize)]
pub struct Config {
    #[serde(default)]
    pub program_includes: Vec<String>,
    /// Publish all accounts on startup.
    #[serde(default)]
    pub publish_all_accounts: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            program_includes: Vec::new(),
            publish_all_accounts: false,
        }
    }
}

impl Config {
    /// Read plugin from JSON file.
    pub fn read_from<P: AsRef<Path>>(config_path: P) -> PluginResult<Self> {
        let file = File::open(config_path)?;
        let this: Self = serde_json::from_reader(file)
            .map_err(|e| AccountsDbPluginError::ConfigFileReadError { msg: e.to_string() })?;
        Ok(this)
    }
}
