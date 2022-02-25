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

use solana_accountsdb_plugin_interface::accountsdb_plugin_interface::SlotStatus as PluginSlotStatus;

include!(concat!(
    env!("OUT_DIR"),
    "/blockdaemon.solana.accountsdb_plugin_kafka.types.rs"
));

impl From<PluginSlotStatus> for SlotStatus {
    fn from(other: PluginSlotStatus) -> Self {
        match other {
            PluginSlotStatus::Processed => SlotStatus::Processed,
            PluginSlotStatus::Rooted => SlotStatus::Rooted,
            PluginSlotStatus::Confirmed => SlotStatus::Confirmed,
        }
    }
}
