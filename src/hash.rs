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

pub use ethereum_types::{H128, H160, H256, H264, H512, H520};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use ethereum_types_serialize;

macro_rules! impl_serde {
    ($name: ident, $len: expr) => {
        impl Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer, {
                let mut slice = [0u8; 2 + 2 * $len];
                ethereum_types_serialize::serialize(&mut slice, &self.0, serializer)
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>, {
                let mut bytes = [0u8; $len];
                ethereum_types_serialize::deserialize_check_len(
                    deserializer,
                    ethereum_types_serialize::ExpectedLen::Exact(&mut bytes),
                )?;
                Ok($name(bytes))
            }
        }
    };
}

construct_hash!(H384, 48);
construct_hash!(H768, 96);

impl_serde!(H384, 48);
impl_serde!(H768, 96);