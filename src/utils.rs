/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

use crate::prompt::PromptConfig;

use lazy_static::lazy_static;
use serde_derive::Deserialize;

const BASE_CONFIG: &str = r#"
# This is the config file for vsh. For now you can only edit the Prompt styling here

# Prompt
# These are the default values
# [prompt]

# Whether the prompt is single-lined or double lined
# double = false

# Prompt Background
# Highest value can be 255, anything above that will result in an error.
# If there are mroe than 3 elements that will also result in an error
# color = [115, 147, 179]

# Prompt Text Background
# Same as Prompt Background just that its for Text
# text_color = [33, 33, 33]

# Prompt Character
# The reason I included this is because it gives some cusomization to Classic Prompt users
# The character used at the start of prompt
# Doesn't have to be a char, can also be a String
# promptchar = "λ"

# Style
# Two option: Modern, Classic
# Modern requires you to have nerd fonts and you can change Background color
# Classic has the regular fonts.
# style = "classic"

"#;

#[derive(Deserialize)]
pub struct Config {
    pub prompt: Option<PromptConfig>,
    pub misc: Option<Misc>,
}

#[derive(Deserialize)]
pub struct Misc {
    pub alias: Option<Vec<[String; 2]>>,
}

pub fn fetch_data() -> String {
    let mut path = PathBuf::from(env::var("HOME").unwrap());
    path.push(".vshrc.toml");

    let mut data = String::new();
    if path.exists() {
        match File::open(&path) {
            Ok(mut x) => {
                if x.read_to_string(&mut data).is_err() {
                    eprintln!("vsh: config file is not in UTF-8 encoding and cannot be read");
                }
            }
            Err(_) => {
                eprintln!("vsh: Error Occured while opening config file");
            }
        }
    } else {
        match File::create(&path) {
            Ok(mut x) => {
                if x.write_all(BASE_CONFIG.as_bytes()).is_err() {
                    eprintln!("vsh: Could not write to config file")
                }
                data = String::from("");
            }
            Err(_) => eprintln!("vsh: Config File could not be created!"),
        }
    }
    data
}

pub fn get_toml(data: String) -> Result<Config, String> {
    match toml::from_str::<Config>(&data) {
        Ok(ok) => Ok(ok),
        Err(e) => Err(e.to_string()),
    }
}

pub fn get_alias(data: &Config) -> HashMap<&str, &str> {
    let mut list: HashMap<&str, &str> = HashMap::new();
    if let Some(misc) = &data.misc {
        if let Some(alias) = &misc.alias {
            for x in alias.iter() {
                list.insert(&x[0], &x[1]);
            }
        }
    }
    list
}

pub fn expand(raw: String) -> String {
    lazy_static! {
        static ref RE: fancy_regex::Regex = fancy_regex::Regex::new("(?<!\\\\)\\~").unwrap();
    }
    RE.replace_all(&raw, env::var("HOME").unwrap()).to_string()
}
