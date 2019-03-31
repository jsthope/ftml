/*
 * parse/mod.rs
 *
 * wikidot-html - Library to convert Wikidot syntax into HTML
 * Copyright (c) 2019 Ammon Smith for Project Foundation
 *
 * wikidot-html is available free of charge under the terms of the MIT
 * License. You are free to redistribute and/or modify it under those
 * terms. It is distributed in the hopes that it will be useful, but
 * WITHOUT ANY WARRANTY. See the LICENSE file for more details.
 *
 */

pub mod regex;

use super::prelude::*;

// TODO
pub fn parse(text: &str) -> Result<SyntaxTree> {
    Err(Error::StaticMsg("Not implemented yet"))
}
