// Copyright 2018 Kodebox, Inc.
// This file is part of CodeChain.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use ckey::PlatformAddress;

use super::super::uint::Uint;

/// Tendermint params deserialization.
#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TendermintParams {
    /// Valid validators.
    pub validators: Vec<PlatformAddress>,
    /// Propose step timeout in milliseconds.
    pub timeout_propose: Option<Uint>,
    /// Prevote step timeout in milliseconds.
    pub timeout_prevote: Option<Uint>,
    /// Precommit step timeout in milliseconds.
    pub timeout_precommit: Option<Uint>,
    /// Commit step timeout in milliseconds.
    pub timeout_commit: Option<Uint>,
    /// Reward per block.
    pub block_reward: Option<Uint>,
}

/// Tendermint engine deserialization.
#[derive(Debug, PartialEq, Deserialize)]
pub struct Tendermint {
    pub params: TendermintParams,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use ckey::PlatformAddress;
    use serde_json;

    use super::Tendermint;

    #[test]
    fn tendermint_deserialization() {
        let s = r#"{
            "params": {
                "validators": ["tccqzzpxln6w5zrhmfju3zc53w6w4y6s95mf5hw0n62"]
            }
        }"#;

        let deserialized: Tendermint = serde_json::from_str(s).unwrap();
        let vs = vec![PlatformAddress::from_str("tccqzzpxln6w5zrhmfju3zc53w6w4y6s95mf5hw0n62").unwrap()];
        assert_eq!(deserialized.params.validators, vs);
    }
}
