// Copyright 2023 Fondazione LINKS

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.



use thiserror::Error;

/// List of errors that could happen
#[derive(Error, Debug)]
pub enum Error {
    #[error("This is not an Object, cannot be flattened!")]
    NotAnObject,

    #[error("This should be a Value")]
    NotAValue,

    #[error("The property is not valid")]
    InvalidProperty,

    #[error("mixed type array")]
    MixedTypeArray,

    #[error("This should be an Object or an Array")]
    InvalidType,

    #[error("Unknown Error")]
    Unspecified,

    #[error("JSON format error")]
    FormatError,

}