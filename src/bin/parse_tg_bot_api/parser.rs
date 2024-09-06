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


use std::collections::{HashSet, HashMap, BTreeSet};

use anyhow::{bail, Context, Result};
use rayon::prelude::*;
use select::{
  document::Document,
  node::Node,
  predicate::{Attr, Class},
};
use serde::de::value;

use crate::tg_api::{Type, Method, Field};


pub(crate) enum Tag {
  H4Tag(H4Tag),
  PTag(PTag),
  TableTag(TableTag),
  UlTag(UlTag),
}


impl Default for Tag {
  fn default() -> Self {
    Self::H4Tag(H4Tag::default())
  }
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
      "h4" => {
        if node.text().contains(" ") {
          continue
        }
        Tag::H4Tag(parse_tag_h4(&node))
      },

      "p" => Tag::PTag(parse_tag_p(&node)),

      "table" => {
        if node.attr("class").context("ERROR: The table tag does not have the class attribute")? != "table" {
          continue
        }
        Tag::TableTag(parse_tag_table(&node)?)
      },

      "ul" => Tag::UlTag(parse_tag_ul(&node)?),
      _ => continue,
    };

    result.push(tag);
  }

  Ok(result)
}


pub(crate) fn parse_api(tags: &Vec<Tag>) -> Result<(HashSet<Type>, HashSet<Method>)> {
  let (types, methods): (Result<HashSet<Type>>, HashSet<Method>) = rayon::join(
    || -> Result<HashSet<Type>> { Ok(parse_types(tags)?) },
    || -> HashSet<Method> { parse_methods(tags) },
  );
  Ok((types?, methods))
}


#[derive(Clone)]
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


impl Default for H4Tag {
  fn default() -> Self {
    Self::new(String::default())
  }
}


#[derive(Clone)]
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


#[derive(Clone)]
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


#[derive(Clone)]
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


#[derive(Clone)]
pub(crate) struct UlTag {
  pub(crate) list_items: HashSet<LiTag>,
}


impl UlTag {
  pub(crate) fn new(list_items: HashSet<LiTag>) -> Self {
    Self {
      list_items,
    }
  }
}


#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub(crate) struct LiTag {
  pub(crate) value: String,
}


impl LiTag {
  pub(crate) fn new(value: String) -> Self {
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


fn parse_tag_ul(node: &Node) -> Result<UlTag> {
  let mut list_items: HashSet<LiTag> = HashSet::new();

  for tag in node.children() {
    let tag_name: &str = match tag.name() {
      Some(name) => name,
      None => continue,
    };

    if tag_name != "li" {
      continue;
    }

    list_items.insert(LiTag::new(tag.text().trim().to_string()));
  }

  Ok(UlTag::new(list_items))
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
  let mut result: HashSet<Type> = HashSet::new();

  let mut prev_tag: Tag = Tag::default();
  let mut type_name: String = String::new();
  let mut type_desc: String = String::new();
  
  for tag in tags {
    match tag {
      Tag::H4Tag(tag) => {
        if let Tag::PTag(_) = prev_tag {
          match type_name.chars().next() {
            Some(ch) => {
              if ch.is_uppercase() {
                result.insert(parse_type(&type_name, &type_desc, None, None)?);
              }
            },
            None => (),
          }
        }

        type_name = tag.value.clone();
        prev_tag = Tag::H4Tag(tag.clone());
      },

      Tag::PTag(tag) => {
        type_desc = tag.value.clone();
        prev_tag = Tag::PTag(tag.clone());
      },

      Tag::TableTag(tag) => {
        if type_name.chars().next().context("ERROR: Empty type name")?.is_uppercase() {
          result.insert(parse_type(&type_name, &type_desc, Some(tag), None)?);
        }
        prev_tag = Tag::TableTag(tag.clone());
      },

      Tag::UlTag(tag) => {
        match type_name.chars().next() {
          Some(ch) => {
            if ch.is_uppercase() {
              result.insert(parse_type(&type_name, &type_desc, None, Some(tag))?);
            }
          },
          None => (),
        }
        prev_tag = Tag::UlTag(tag.clone());
      },
    }
  }
  
  Ok(result)
}


fn parse_methods(tags: &Vec<Tag>) -> HashSet<Method> {
  HashSet::new()
}


fn parse_type(name: &String, desc: &String, table: Option<&TableTag>, ul: Option<&UlTag>) -> Result<Type> {
  if table.is_some() && ul.is_some() {
    bail!("ERROR: Type can only have one of 'table' or 'ul'");
  }

  let mut fields: BTreeSet<Field> = match table {
    Some(table) => get_fields_from_table(table)?,
    None => BTreeSet::new(),
  };

  fields = match ul {
    Some(ul) => get_fields_from_ul(ul)?,
    None => fields,
  };
  
  Ok(Type::new(name.clone(), desc.clone(), fields))
}


fn get_fields_from_table(table: &TableTag) -> Result<BTreeSet<Field>> {
  let mut result: BTreeSet<Field> = BTreeSet::new();

  for line in &table.lines {
    let name: String = line.value.get("Field").context("ERROR: The field did not have a name found")?.clone();
    let r#type: String = line.value.get("Type").context("ERROR: The field type was not found")?.clone();
    let description: String = line.value.get("Description").context("ERROR: No description found for the field")?.clone();

    let r#type: String = parse_field_type(&r#type);

    result.insert(Field::new(name, r#type, description.starts_with("Optional"), description));
  }

  Ok(result)
}


fn get_fields_from_ul(ul: &UlTag) -> Result<BTreeSet<Field>> {
  let mut result: BTreeSet<Field> = BTreeSet::new();

  for li in &ul.list_items {
    result.insert(Field::new(li.value.clone(), li.value.clone(), false, String::from("")));
  }

  Ok(result)
}


fn parse_field_type(type_name: &String) -> String {
  if type_name.trim().starts_with("Array of") {
    return format!("Vec<{}>", parse_field_type(&type_name.split_at("Array of".len()).1.trim().to_string()));
  }

  let tg_types: HashMap<String, String> = HashMap::from([
    ("Integer".to_string(), "i64".to_string()),
    ("True".to_string(), "bool".to_string()),
    ("Boolean".to_string(), "bool".to_string()),
    ("Float".to_string(), "f64".to_string()),
    ("InputFile or String".to_string(), "String".to_string()),
    ("Integer or String".to_string(), "String".to_string()),
  ]);

  match tg_types.get(type_name) {
    Some(r#type) => r#type.clone(),
    None => type_name.clone(),
  }
}
