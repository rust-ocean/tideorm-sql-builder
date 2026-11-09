//! Helper for preparing SQL statements.

pub(crate) mod parameters;

use crate::error::RenderError;
use crate::*;
pub use std::fmt::Write;

pub trait SqlWriter: Write + Sized + ToString {
    /// Writes `value`'s marker and collects it, or, for a `String`, writes
    /// the value itself, which fails for a value the backend cannot write.
    fn push_param<T: QueryBuilder>(
        &mut self,
        value: Value,
        query_builder: &T,
    ) -> Result<(), RenderError>;
}

impl SqlWriter for String {
    fn push_param<T: QueryBuilder>(
        &mut self,
        value: Value,
        query_builder: &T,
    ) -> Result<(), RenderError> {
        query_builder.write_value(self, &value)
    }
}

#[derive(Debug, Clone)]
pub struct SqlWriterValues {
    counter: usize,
    placeholder: String,
    numbered: bool,
    string: String,
    values: Vec<Value>,
}

impl SqlWriterValues {
    pub fn new<T>(placeholder: T, numbered: bool) -> Self
    where
        T: Into<String>,
    {
        Self {
            counter: 0,
            placeholder: placeholder.into(),
            numbered,
            string: String::with_capacity(256),
            values: Vec::new(),
        }
    }

    pub fn into_parts(self) -> (String, Values) {
        (self.string, Values(self.values))
    }
}

impl Write for SqlWriterValues {
    #[inline]
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.string.write_str(s)
    }

    #[inline]
    fn write_char(&mut self, c: char) -> std::fmt::Result {
        self.string.write_char(c)
    }
}

impl std::fmt::Display for SqlWriterValues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.string)
    }
}

impl SqlWriter for SqlWriterValues {
    fn push_param<T: QueryBuilder>(&mut self, value: Value, _: &T) -> Result<(), RenderError> {
        self.string.push_str(&self.placeholder);
        if self.numbered {
            self.counter += 1;
            write_int(&mut self.string, self.counter);
        }
        self.values.push(value);
        Ok(())
    }
}

#[inline(always)]
pub(crate) fn write_int(w: &mut (impl Write + ?Sized), n: impl std::fmt::Display) {
    write!(w, "{n}").unwrap();
}

pub fn inject_parameters(
    sql: &str,
    params: &[Value],
    query_builder: &impl QueryBuilder,
) -> Result<String, RenderError> {
    let tokens = parameters::parameter_tokens(sql, params.len(), query_builder, false)?;
    let mut output = String::with_capacity(sql.len());
    for token in tokens {
        match token {
            parameters::ParameterToken::Text(text) => output.push_str(text),
            parameters::ParameterToken::Parameter(index) => {
                query_builder.write_value(&mut output, &params[index])?;
            }
        }
    }
    Ok(output)
}

#[cfg(test)]
#[path = "../tests/unit/prepare_mysql_tests.rs"]
mod tests_mysql;
#[cfg(test)]
#[path = "../tests/unit/prepare_postgres_tests.rs"]
mod tests_postgres;
