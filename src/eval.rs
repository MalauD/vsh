/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use crate::commands;
use std::string::ToString;

pub struct Internalcommand {
    keyword: String,
    args: Vec<String>,
}

pub enum CommandError {
    Error,
    Exit,
    Finished(i32),   // If the program finished with a non-zero exit code
    Terminated(i32), // If the program was terminated by the user
}

impl Internalcommand {
    pub fn new(input: String) -> Self {
        let mut splitted = input.trim().split_whitespace();
        let keyword = match splitted.next() {
            Some(x) => x.to_string(),
            None => String::from(""),
        };
        Self {
            keyword,
            args: splitted.map(ToString::to_string).collect::<Vec<String>>(),
        }
    }

    pub fn eval(&mut self) -> Result<(), CommandError> {
        match (self.keyword.as_str(), self.args.clone()) {
            ("cd", args) => commands::cd(args.get(0)),
            ("", _) => println!(),
            ("exit", _) => {
                return Err(CommandError::Exit);
            }
            (x, args) => match *x.as_bytes().last().unwrap() as char {
                '/' => commands::cd(Some(&x.to_string())),
                _ => {
                    return commands::neutral(x.to_string(), args);
                }
            },
        }
        Ok(())
    }
}
