use crate::{
    Expr, QueryBuilder,
    error::RenderError,
    prepare::parameters::{ParameterToken, parameter_tokens},
};

/// A custom fragment whose markers have been matched to expressions.
#[derive(Debug, Clone, PartialEq)]
pub struct CustomExpr {
    pub(crate) parts: Vec<CustomExprPart>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum CustomExprPart {
    Text(String),
    Expression(Expr),
}

impl CustomExpr {
    pub(crate) fn new(
        sql: &str,
        values: Vec<Expr>,
        builder: &impl QueryBuilder,
    ) -> Result<Self, RenderError> {
        let parts = parameter_tokens(sql, values.len(), builder, true)?
            .into_iter()
            .map(|token| match token {
                ParameterToken::Text(text) => CustomExprPart::Text(text.to_owned()),
                ParameterToken::Parameter(index) => {
                    CustomExprPart::Expression(values[index].clone())
                }
            })
            .collect();
        Ok(Self { parts })
    }
}
