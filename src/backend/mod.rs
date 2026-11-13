//! Translating the SQL AST into engine-specific SQL statements.

use crate::*;
use std::borrow::Cow;

mod mysql;

mod postgres;

mod sqlite;

pub use mysql::*;

pub use postgres::*;

pub use sqlite::*;

mod bound_sql;
mod dialect;
mod json_path;
mod query_builder;
mod table_ref_builder;

pub use self::dialect::{
    ArrayOperator, BoundPart, BoundPiece, BoundSql, JsonContainment, JsonExistence, MATCH_NOTHING,
    SqlDialect,
};
pub use self::json_path::{JsonPathStep, is_plain_identifier, json_path_text, parse_json_path};
pub use self::query_builder::*;
pub use self::table_ref_builder::*;

pub trait QuotedBuilder {
    /// The type of quote the builder uses.
    fn quote(&self) -> Quote;

    /// To prepare iden and write to SQL.
    fn prepare_iden(&self, iden: &DynIden, sql: &mut impl SqlWriter) {
        let quote = self.quote();
        match &iden.0 {
            Cow::Borrowed(name) => {
                sql.write_char(quote.left()).unwrap();
                sql.write_str(name).unwrap();
                sql.write_char(quote.right()).unwrap();
            }
            Cow::Owned(name) => write_identifier(sql, quote, name),
        }
    }
}

/// Writes `name` between `quote`s, doubling every closing quote inside it.
pub(crate) fn write_identifier(out: &mut impl Write, quote: Quote, name: &str) {
    out.write_char(quote.left()).unwrap();
    for ch in name.chars() {
        if ch == quote.right() {
            out.write_char(ch).unwrap();
        }
        out.write_char(ch).unwrap();
    }
    out.write_char(quote.right()).unwrap();
}

pub trait EscapeBuilder {
    /// Return if string literal needs to be escaped
    fn needs_escape(&self, s: &str) -> bool {
        s.chars().any(|c| {
            matches!(
                c,
                '\r' | '\n' | '\x1a' | '\x09' | '\x08' | '\0' | '\'' | '"' | '\\'
            )
        })
    }

    /// Escape a SQL string literal
    fn escape_string(&self, string: &str) -> String {
        let mut escaped = String::with_capacity(string.len() + 8);
        self.write_escaped(&mut escaped, string);
        escaped
    }

    fn write_escaped(&self, buffer: &mut impl Write, string: &str) {
        for c in string.chars() {
            match c {
                '\\' => buffer.write_str("\\\\"),
                '"' => buffer.write_str("\\\""),
                // doubled, not `\'`: MySQL reads `''` the same with
                // NO_BACKSLASH_ESCAPES, where `\'` would end the literal
                '\'' => buffer.write_str("''"),
                '\0' => buffer.write_str("\\0"),
                '\x08' => buffer.write_str("\\b"),
                '\x09' => buffer.write_str("\\t"),
                '\x1a' => buffer.write_str("\\z"),
                '\n' => buffer.write_str("\\n"),
                '\r' => buffer.write_str("\\r"),
                _ => buffer.write_char(c),
            }
            .unwrap()
        }
    }
}

pub trait PrecedenceDecider {
    // This method decides which precedence relations should lead to dropped parentheses.
    // There will be more fine grained precedence relations than the ones represented here,
    // but dropping parentheses due to these relations can be confusing for readers.
    fn inner_expr_well_known_greater_precedence(&self, inner: &Expr, outer_oper: &Oper) -> bool {
        common_inner_expr_well_known_greater_precedence(inner, outer_oper)
    }
}

pub trait OperLeftAssocDecider {
    // This method decides if the left associativity of an operator should lead to dropped parentheses.
    // Not all known left associative operators are necessarily included here,
    // as dropping them may in some cases be confusing to readers.
    fn well_known_left_associative(&self, op: &BinOper) -> bool {
        common_well_known_left_associative(op)
    }
}

#[derive(Debug, PartialEq)]
pub enum Oper {
    UnOper(UnOper),
    BinOper(BinOper),
}

impl From<UnOper> for Oper {
    fn from(value: UnOper) -> Self {
        Oper::UnOper(value)
    }
}

impl From<BinOper> for Oper {
    fn from(value: BinOper) -> Self {
        Oper::BinOper(value)
    }
}

impl Oper {
    pub(crate) fn is_logical(&self) -> bool {
        matches!(
            self,
            Oper::UnOper(UnOper::Not) | Oper::BinOper(BinOper::And) | Oper::BinOper(BinOper::Or)
        )
    }

    pub(crate) fn is_between(&self) -> bool {
        matches!(
            self,
            Oper::BinOper(BinOper::Between) | Oper::BinOper(BinOper::NotBetween)
        )
    }

    pub(crate) fn is_like(&self) -> bool {
        matches!(
            self,
            Oper::BinOper(BinOper::Like) | Oper::BinOper(BinOper::NotLike)
        )
    }

    pub(crate) fn is_in(&self) -> bool {
        matches!(
            self,
            Oper::BinOper(BinOper::In) | Oper::BinOper(BinOper::NotIn)
        )
    }

    pub(crate) fn is_is(&self) -> bool {
        matches!(
            self,
            Oper::BinOper(BinOper::Is) | Oper::BinOper(BinOper::IsNot)
        )
    }

    pub(crate) fn is_shift(&self) -> bool {
        matches!(
            self,
            Oper::BinOper(BinOper::LShift) | Oper::BinOper(BinOper::RShift)
        )
    }

    pub(crate) fn is_arithmetic(&self) -> bool {
        match self {
            Oper::BinOper(b) => {
                matches!(
                    b,
                    BinOper::Mul | BinOper::Div | BinOper::Mod | BinOper::Add | BinOper::Sub
                )
            }
            _ => false,
        }
    }

    pub(crate) fn is_comparison(&self) -> bool {
        match self {
            Oper::BinOper(b) => {
                matches!(
                    b,
                    BinOper::SmallerThan
                        | BinOper::SmallerThanOrEqual
                        | BinOper::Equal
                        | BinOper::GreaterThanOrEqual
                        | BinOper::GreaterThan
                        | BinOper::NotEqual
                )
            }
            _ => false,
        }
    }
}
