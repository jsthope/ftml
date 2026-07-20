/*
 * render/html/element/style.rs
 *
 * ftml - Library to parse Wikidot text
 * Copyright (C) 2019-2026 Wikijump Team
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
use lightningcss::stylesheet::{ParserOptions, PrinterOptions, StyleSheet};

/// Prevent CSS from terminating the HTML `<style>` raw-text element.
///
/// This must run after CSS serialization: HTML parsing happens before the
/// browser parses the CSS, so a CSS escape is needed rather than HTML
/// escaping. The trailing space terminates the hexadecimal CSS escape.
fn escape_style_end_tags(css: &str) -> String {
    const END_TAG: &[u8] = b"</style";

    let bytes = css.as_bytes();
    let mut escaped = String::with_capacity(css.len());
    let mut cursor = 0;

    while cursor < bytes.len() {
        let matches = cursor + END_TAG.len() <= bytes.len()
            && bytes[cursor..cursor + END_TAG.len()]
                .iter()
                .zip(END_TAG.iter())
                .all(|(&actual, &expected)| actual.eq_ignore_ascii_case(&expected));

        if matches {
            escaped.push_str(r"\3c /style");
            cursor += END_TAG.len();
        } else {
            // `css` is valid UTF-8, so copying the next character rather than
            // a byte keeps the result valid when non-ASCII CSS is present.
            let character = css[cursor..].chars().next().expect("valid UTF-8");
            escaped.push(character);
            cursor += character.len_utf8();
        }
    }

    escaped
}

pub fn render_style(ctx: &mut HtmlContext, input_css: &str) {
    let minify = ctx.settings().minify_css;

    let parser_options = ParserOptions {
        error_recovery: true,
        ..Default::default()
    };

    let print_options = PrinterOptions {
        minify,
        ..Default::default()
    };

    debug!("Parsing input CSS ({} bytes)", input_css.len());
    let stylesheet = StyleSheet::parse(input_css, parser_options)
        .expect("Produced error with recovery enabled");

    trace!("Rendering CSS into HTML (minify: {minify})");
    let output_css = match stylesheet.to_css(print_options) {
        Ok(output) => output.code,
        Err(error) => {
            error!("Problem outputting CSS from stylesheet: {error}");
            trace!("Input CSS:\n{input_css}");
            trace!("Parsed stylesheet:\n{stylesheet:#?}");
            return;
        }
    };

    let output_css = escape_style_end_tags(&output_css);

    ctx.html().style().inner(|ctx| {
        // SAFETY: `escape_style_end_tags` prevents CSS from closing the
        //         surrounding HTML style element before raw insertion.
        ctx.push_raw_str(&output_css);
    });
}

#[cfg(test)]
mod tests {
    use super::escape_style_end_tags;

    #[test]
    fn escapes_style_end_tags_case_insensitively() {
        assert_eq!(
            escape_style_end_tags(r#"content: "</style><script>";"#),
            r#"content: "\3c /style><script>";"#,
        );
        assert_eq!(
            escape_style_end_tags(r#"content: "</STYLE><script>";"#),
            r#"content: "\3c /style><script>";"#,
        );
        assert_eq!(
            escape_style_end_tags(r#"content: "</StYlE >";"#),
            r#"content: "\3c /style >";"#,
        );
    }

    #[test]
    fn leaves_other_css_untouched() {
        let css = r#"@media (width < 600px) { x { content: "</st yle>"; } }"#;
        assert_eq!(escape_style_end_tags(css), css);
    }
}
