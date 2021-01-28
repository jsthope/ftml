/*
 * parsing/rule/impls/block/blocks/radio.rs
 *
 * ftml - Library to parse Wikidot text
 * Copyright (C) 2019-2021 Wikijump Team
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

use super::prelude::*;

pub const BLOCK_RADIO: BlockRule = BlockRule {
    name: "block-radio",
    accepts_names: &["radio", "radio-button"],
    accepts_special: false,
    accepts_newlines: false,
    parse_fn,
};

fn parse_fn<'r, 't>(
    log: &slog::Logger,
    parser: &mut Parser<'r, 't>,
    name: &'t str,
    special: bool,
    in_head: bool,
) -> ParseResult<'r, 't, Element<'t>> {
    debug!(
        log,
        "Parsing radio button block";
        "in-head" => in_head,
        "name" => name,
    );

    assert_eq!(special, false, "Radio buttons don't allow special variant");
    assert_block_name(&BLOCK_RADIO, name);

    let (name, arguments) = parser.get_head_name_map(&BLOCK_RADIO, in_head)?;
    let element = Element::RadioButton {
        name: cow!(name),
        attributes: arguments.to_hash_map(),
    };

    ok!(element)
}
