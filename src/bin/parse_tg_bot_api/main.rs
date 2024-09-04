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


mod tg_api;
mod parser;


use std::collections::HashSet;

use anyhow::{Result, bail};
use reqwest::Response;
use select::{
  document::Document,
  node::Node,
};

use crate::tg_api::{Type, Method};
use crate::parser::Tag;


#[tokio::main]
async fn main() {
  match main_wraper().await {
    Ok(_) => println!("PARSE SUNCCESS!"),
    Err(e) => eprintln!("{e}"),
  }
}


async fn main_wraper() -> Result<()> {
  let html: String = get_html().await?;
  let document: Document = Document::from(html.as_str());
  let tags: Vec<Tag> = parser::get_list_of_main_tags(&document)?;
  let (types, methods): (HashSet<Type>, HashSet<Method>) = parser::parse_api(&tags)?;

  for i in tags {
    match i {
      Tag::H4Tag(tag) => println!("{:?}", tag.value),
      Tag::PTag(tag) => println!("{:?}", tag.value),
      Tag::TableTag(tag) => {
        for line in tag.lines {
          println!("{:?}", line.value);
        }
      },
    }
  }

  Ok(())
}


async fn get_html() -> Result<String> {
  let url: String = String::from("https://core.telegram.org/bots/api");
  let response: Response = reqwest::get(&url).await?;
  
  if !response.status().is_success() {
    bail!("ERROR: Request to {} failed with {}", url, response.status());
  }

  let html: String = response.text().await?;
  Ok(html)
}
