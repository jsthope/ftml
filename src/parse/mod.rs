/*
 * parse/mod.rs
 *
 * ftml - Library to parse Wikidot code
 * Copyright (C) 2019-2020 Ammon Smith
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

mod consume;
mod error;
mod result;
mod rule;
mod token;

#[cfg(test)]
mod test;

use self::consume::consume;
use self::rule::Consumption;
use crate::tokenize::Tokenization;
use crate::tree::SyntaxTree;

pub use self::error::{ParseError, ParseErrorKind};
pub use self::result::ParseResult;
pub use self::token::{ExtractedToken, Token};

/// Parse through the given tokens and produce an AST.
///
/// This takes a list of `ExtractedToken` items produced by `tokenize()`.
pub fn parse<'r, 't>(
    log: &slog::Logger,
    tokenization: &'r Tokenization<'t>,
) -> ParseResult<SyntaxTree<'t>>
where
    'r: 't,
{
    // Set up variables
    let mut output = ParseResult::default();
    let mut tokens = tokenization.tokens();
    let full_text = tokenization.full_text();

    // Logging setup
    let log = &log.new(slog_o!(
        "filename" => slog_filename!(),
        "lineno" => slog_lineno!(),
        "function" => "parse",
        "tokens-len" => tokens.len(),
    ));

    // Run through tokens until finished
    info!(log, "Running parser on tokens");

    while !tokens.is_empty() {
        // Consume tokens to produce the next element
        let consumption = {
            let (extracted, remaining) = tokens
                .split_first() //
                .expect("Tokens list is empty");

            consume(log, extracted, remaining, full_text)
        };

        let error = match consumption {
            Consumption::Success {
                item,
                remaining,
                error,
            } => {
                debug!(log, "Tokens successfully consumed to produce element");

                // Update remaining tokens
                //
                // The new value is a subslice of tokens,
                // equivalent to &tokens[offset..] but without
                // needing to assert bounds.
                tokens = remaining;

                // Add the new element to the list
                output.push(item);

                // Return the error, if any
                error
            }
            Consumption::Failure { error } => {
                debug!(log, "Tokens unsuccessfully consumed, no element");

                // Return the error
                Some(error)
            }
        };

        if let Some(error) = error {
            info!(
                log,
                "Received error during token consumption";
                "error-token" => error.token(),
                "error-rule" => error.rule(),
                "error-span-start" => error.span().start,
                "error-span-end" => error.span().end,
                "error-kind" => error.kind().name(),
            );

            output.append_err(error);
        }
    }

    info!(log, "Finished running parser, returning gathered elements");
    SyntaxTree::from_element_result(output)
}
