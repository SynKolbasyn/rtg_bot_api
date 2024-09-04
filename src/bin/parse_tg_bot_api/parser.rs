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


use std::collections::{HashSet, HashMap};

use anyhow::{bail, Context, Result};
use select::{
  document::Document,
  node::Node,
  predicate::Attr,
};

use crate::tg_api::{Type, Method};


pub(crate) enum Tag {
  H4Tag(H4Tag),
  PTag(PTag),
  TableTag(TableTag),
}


pub(crate) fn get_list_of_main_tags(document: &Document) -> Result<Vec<Tag>> {
  let mut result: Vec<Tag> = Vec::new();
  let document: Node = document.find(Attr("id", "dev_page_content")).next().context("ERROR: Couldn't find the start tag of the data")?;

  for node in document.children() {
    let node_name: &str = match node.name() {
      Some(name) => name.trim(),
      None => continue,
    };

    let tag: Tag = match node_name {
      "p" => Tag::H4Tag(parse_tag_h4(&node)),
      "h4" => Tag::PTag(parse_tag_p(&node)),
      "table" => Tag::TableTag(parse_tag_table(&node)?),
      _ => continue,
    };

    result.push(tag);
  }

  Ok(result)
}


pub(crate) fn parse_api(tags: &Vec<Tag>) -> Result<(HashSet<Type>, HashSet<Method>)> {
  Ok((parse_types(tags)?, parse_methods(tags)?))
}


pub(crate) struct H4Tag {
  pub(crate) value: String,
}


impl H4Tag {
  fn new(value: String) -> Self {
    Self {
      value,
    }
  }
}


pub(crate) struct PTag {
  pub(crate) value: String,
}


impl PTag {
  fn new(value: String) -> Self {
    Self {
      value,
    }
  }
}


pub(crate) struct TableTag {
  pub(crate) lines: Vec<LineTag>,
}


impl TableTag {
  fn new(lines: Vec<LineTag>) -> Self {
    Self {
      lines,
    }
  }
}


pub(crate) struct LineTag {
  pub(crate) value: HashMap<String, String>,
}


impl LineTag {
  fn new(value: HashMap<String, String>) -> Self {
    Self {
      value,
    }
  }
}


fn parse_tag_h4(node: &Node) -> H4Tag {
  H4Tag::new(node.text())
}


fn parse_tag_p(node: &Node) -> PTag {
  PTag::new(node.text())
}


fn parse_tag_table(node: &Node) -> Result<TableTag> {
  let mut column_names: Vec<String> = Vec::new();
  let mut lines: Vec<LineTag> = Vec::new();

  for tag in node.children() {
    let tag_name: &str = match tag.name() {
      Some(name) => name,
      None => continue,
    };

    match tag_name {
      "thead" => column_names = parse_table_thead(&tag)?,
      "tbody" => lines = parse_table_tbody(&tag, &column_names)?,
      _ => (),
    }
  }

  Ok(TableTag::new(lines))
}


fn parse_table_thead(node: &Node) -> Result<Vec<String>> {
  let mut result: Vec<String> = Vec::new();

  for tag in node.children() {
    let tag_name: &str = match tag.name() {
      Some(name) => name,
      None => continue,
    };

    if tag_name != "tr" {
      continue;
    }

    for column in tag.children() {
      let column_name: &str = match column.name() {
        Some(name) => name,
        None => continue,
      };

      if column_name != "th" {
        continue;
      }

      result.push(column.text().trim().to_string());
    }
  }

  Ok(result)
}


fn parse_table_tbody(node: &Node, column_name: &Vec<String>) -> Result<Vec<LineTag>> {
  let mut result: Vec<LineTag> = Vec::new();

  for tag in node.children() {
    let tag_name: &str = match tag.name() {
      Some(name) => name,
      None => continue,
    };

    if tag_name != "tr" {
      continue;
    }

    let mut line: HashMap<String, String> = HashMap::new();
    let mut idx: usize = 0;
    for field in tag.children() {
      let field_name: &str = match field.name() {
        Some(name) => name,
        None => continue,
      };

      if field_name != "td" {
        continue;
      }

      line.insert(column_name[idx].clone(), field.text().trim().to_string());
      idx += 1;
    }

    result.push(LineTag::new(line));
  }

  Ok(result)
}


fn parse_types(tags: &Vec<Tag>) -> Result<HashSet<Type>> {
  Ok(HashSet::new())
}


fn parse_methods(tags: &Vec<Tag>) -> Result<HashSet<Method>> {
  Ok(HashSet::new())
}
