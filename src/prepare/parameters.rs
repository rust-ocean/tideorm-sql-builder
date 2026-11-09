use crate::{
    QueryBuilder,
    error::RenderError,
    lexer::{Kind, tokens},
};

pub(crate) enum ParameterToken<'a> {
    Text(&'a str),
    Parameter(usize),
}

/// `sql` split at its markers, read under `builder`'s lexical rules, so a
/// marker inside a string, a quoted name or a comment is text. Each value of
/// `count` must be used. With `escaped_markers`, a doubled `?` marker is one
/// `?` of text.
pub(crate) fn parameter_tokens<'a>(
    sql: &'a str,
    count: usize,
    builder: &impl QueryBuilder,
    escaped_markers: bool,
) -> Result<Vec<ParameterToken<'a>>, RenderError> {
    let (placeholder, numbered) = builder.placeholder();
    let mut tokens = tokens(sql, builder.lexer_dialect()).peekable();
    let mut used = vec![false; count];
    let mut positional = 0;
    let mut output = Vec::new();
    while let Some(token) = tokens.next() {
        let index = if numbered && token.kind == Kind::Parameter {
            let number = token.text.strip_prefix(placeholder).unwrap_or(token.text);
            number
                .parse::<usize>()
                .ok()
                .and_then(|n| n.checked_sub(1))
                .ok_or_else(|| RenderError::InvalidIndex(number.to_owned()))?
        } else if token.kind == Kind::Symbol && token.text == placeholder {
            if numbered {
                return Err(RenderError::InvalidIndex(String::new()));
            }
            if escaped_markers
                && tokens
                    .peek()
                    .is_some_and(|next| next.kind == Kind::Symbol && next.text == placeholder)
            {
                tokens.next();
                output.push(ParameterToken::Text(token.text));
                continue;
            }
            positional += 1;
            positional - 1
        } else {
            output.push(ParameterToken::Text(token.text));
            continue;
        };
        let slot = used
            .get_mut(index)
            .ok_or_else(|| RenderError::MissingValue {
                index: index.saturating_add(1),
                count,
            })?;
        *slot = true;
        output.push(ParameterToken::Parameter(index));
    }
    if let Some(index) = used.iter().position(|used| !used) {
        return Err(RenderError::UnusedValue { index: index + 1 });
    }
    Ok(output)
}
