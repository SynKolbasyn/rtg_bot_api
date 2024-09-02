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


use std::collections::HashSet;

use anyhow::{Result, Context};
use select::{
  document::Document,
  node::Node,
  predicate::Attr,
};

use crate::tg_api::{Type, Method};


pub(crate) struct H4Tag {
  value: String,
}


pub(crate) struct PTag {
  value: String,
}


pub(crate) struct TableTag {
  lines: Vec<LineTag>,
}


pub(crate) struct LineTag {
  value: Vec<String>,
}


pub(crate) fn get_list_of_main_tags(document: &Document) -> Result<Vec<Node>> {
  let mut result: Vec<Node> = Vec::new();
  let necessary_tags: HashSet<&str> = HashSet::from(["h4", "p", "table"]);
  let document: Node = document.find(Attr("id", "dev_page_content")).next().context("ERROR: Couldn't find the start tag of the data")?;

  for tag in document.children() {
    let tag_name: &str = match tag.name() {
      Some(name) => name,
      None => continue,
    };

    if !necessary_tags.contains(tag_name) {
      continue;
    }

    result.push(tag);
  }

  Ok(result)
}


pub(crate) fn parse_api(tags: &Vec<Node>) -> Result<(HashSet<Type>, HashSet<Method>)> {
  Ok((parse_types(tags)?, parse_methods(tags)?))
}


pub(crate) fn parse_types(tags: &Vec<Node>) -> Result<HashSet<Type>> {
  for tag in tags {
    if tag.name()
  }
}


pub(crate) fn parse_methods(tags: &Vec<Node>) -> Result<HashSet<Method>> {
  Ok(HashSet::new())
}



