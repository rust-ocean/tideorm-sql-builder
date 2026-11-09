//! JSON paths as every backend reads them: `$` followed by member and index
//! steps.

/// One step of a JSON path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JsonPathStep {
    /// The member a key names.
    Member(String),
    /// The array element at an index.
    Element(u64),
}

/// Whether `segment` is a plain identifier: an ASCII letter or `_`, then
/// ASCII letters, digits or `_`.
pub fn is_plain_identifier(segment: &str) -> bool {
    let mut chars = segment.chars();
    match chars.next() {
        Some(ch) if ch == '_' || ch.is_ascii_alphabetic() => {}
        _ => return false,
    }

    chars.all(|ch| ch == '_' || ch.is_ascii_alphanumeric())
}

/// The steps of a JSON path.
///
/// The path is `$` followed by `.key`, `."quoted key"`, `['quoted key']` or
/// `[index]` steps; `None` for anything else, such as a wildcard or an
/// unquoted key that is not a plain identifier.
pub fn parse_json_path(path: &str) -> Option<Vec<JsonPathStep>> {
    let chars: Vec<char> = path.chars().collect();
    if chars.first().copied() != Some('$') {
        return None;
    }

    let mut index = 1;
    let mut steps = Vec::new();
    while index < chars.len() {
        let step = match chars[index] {
            '.' => {
                index += 1;
                match chars.get(index) {
                    Some('"' | '\'') => JsonPathStep::Member(quoted_segment(&chars, &mut index)?),
                    Some(_) => {
                        let start = index;
                        while index < chars.len() && chars[index] != '.' && chars[index] != '[' {
                            index += 1;
                        }
                        let segment: String = chars[start..index].iter().collect();
                        if !is_plain_identifier(&segment) {
                            return None;
                        }
                        JsonPathStep::Member(segment)
                    }
                    None => return None,
                }
            }
            '[' => {
                index += 1;
                let step = match chars.get(index) {
                    Some('"' | '\'') => JsonPathStep::Member(quoted_segment(&chars, &mut index)?),
                    Some(ch) if ch.is_ascii_digit() => {
                        let start = index;
                        while index < chars.len() && chars[index].is_ascii_digit() {
                            index += 1;
                        }
                        let digits: String = chars[start..index].iter().collect();
                        JsonPathStep::Element(digits.parse().ok()?)
                    }
                    _ => return None,
                };
                if chars.get(index) != Some(&']') {
                    return None;
                }
                index += 1;
                step
            }
            _ => return None,
        };
        steps.push(step);
    }

    Some(steps)
}

/// A JSON path as MySQL, MariaDB and SQLite read it, every key quoted.
pub fn json_path_text(steps: &[JsonPathStep]) -> String {
    steps
        .iter()
        .fold(String::from("$"), |path, step| match step {
            JsonPathStep::Member(key) => member_path(&path, key),
            JsonPathStep::Element(index) => format!("{path}[{index}]"),
        })
}

/// The JSON path of the member `key` below `path`, the key quoted.
pub(crate) fn member_path(path: &str, key: &str) -> String {
    format!("{path}.{}", serde_json::Value::from(key))
}

/// The quoted key starting at `index`, unescaped; `index` moves past it.
fn quoted_segment(chars: &[char], index: &mut usize) -> Option<String> {
    let quote = chars.get(*index).copied()?;
    *index += 1;
    let mut json = String::from("\"");
    while *index < chars.len() {
        let ch = chars[*index];
        *index += 1;
        if ch == quote {
            json.push('"');
            return serde_json::from_str(&json).ok();
        }
        if ch == '\\' {
            let escaped = *chars.get(*index)?;
            *index += 1;
            if quote == '\'' && escaped == '\'' {
                json.push('\'');
            } else {
                json.push('\\');
                json.push(escaped);
            }
        } else {
            if ch == '"' {
                json.push('\\');
            }
            json.push(ch);
        }
    }
    None
}
