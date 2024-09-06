//!   Rust telegram bot api. The library provides asynchronous access to the telegram bot api.
//!   Copyright (C) 2024  Andrew Kozmin
//!
//!   This program is free software: you can redistribute it and/or modify
//!   it under the terms of the GNU Affero General Public License as published by
//!   the Free Software Foundation, either version 3 of the License, or
//!   (at your option) any later version.
//!
//!   This program is distributed in the hope that it will be useful,
//!   but WITHOUT ANY WARRANTY; without even the implied warranty of
//!   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//!   GNU Affero General Public License for more details.
//!
//!   You should have received a copy of the GNU Affero General Public License
//!   along with this program.  If not, see <https://www.gnu.org/licenses/>.


use std::collections::BTreeSet;


#[derive(Debug, Eq, Hash, PartialEq)]
pub(crate) struct Type {
  pub(crate) name: String,
  pub(crate) description: String,
  pub(crate) fields: BTreeSet<Field>,
}


impl Type {
  pub(crate) fn new(name: String, description: String, fields: BTreeSet<Field>) -> Self {
    Self {
      name,
      description,
      fields,
    }
  }
}


pub(crate) struct Method {
  pub(crate) name: String,
  pub(crate) description: String,
  pub(crate) parameters: Vec<Parameter>,
}


#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub(crate) struct Field {
  pub(crate) name: String,
  pub(crate) r#type: String,
  pub(crate) optional: bool,
  pub(crate) description: String,
}


impl Field {
  pub(crate) fn new(name: String, r#type: String, optional: bool, description: String) -> Self {
    Self {
      name,
      r#type,
      optional,
      description,
    }
  }
}


pub(crate) struct Parameter {
  pub(crate) name: String,
  pub(crate) r#type: String,
  pub(crate) required: bool,
  pub(crate) description: String,
}
