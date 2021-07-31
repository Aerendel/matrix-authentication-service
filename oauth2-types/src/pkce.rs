// Copyright 2021 The Matrix.org Foundation C.I.C.
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

use parse_display::{Display, FromStr};
use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(
    Debug,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    Copy,
    Display,
    FromStr,
    Serialize,
    Deserialize,
    Type,
)]
#[repr(i8)]
pub enum CodeChallengeMethod {
    #[serde(rename = "plain")]
    #[display("plain")]
    Plain = 0,

    #[serde(rename = "S256")]
    #[display("S256")]
    S256 = 1,
}

#[derive(Serialize, Deserialize)]
pub struct Request {
    pub code_challenge_method: CodeChallengeMethod,
    pub code_challenge: String,
}
