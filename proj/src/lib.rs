// Copyright 2020 Revcore Technologies Ltd.
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

//! Logging, as well as various low-level utilities that factor Rust
//! patterns that are frequent within the codebase.

#![deny(non_upper_case_globals)]
#![deny(non_camel_case_types)]
#![deny(non_snake_case)]
#![deny(unused_mut)]
#![warn(missing_docs)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate aloevera_util as util;
extern crate aloevera_vera as vera;

mod error;
mod project;
pub use error::{Error, ErrorKind};

pub use project::AloeVeraProject;

/// Just a wrapper around a serializable object
pub trait Jsonable {
	/// to json
	fn to_json(&self) -> Result<String, Error>;
}
