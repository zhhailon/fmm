use super::expression::Expression;
use super::primitive_operator::PrimitiveOperator;
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveOperation {
    operator: PrimitiveOperator,
    lhs: Arc<Expression>,
    rhs: Arc<Expression>,
    name: String,
}

impl PrimitiveOperation {
    pub fn new(
        operator: PrimitiveOperator,
        lhs: impl Into<Expression>,
        rhs: impl Into<Expression>,
        name: impl Into<String>,
    ) -> Self {
        Self {
            operator,
            lhs: Arc::new(lhs.into()),
            rhs: Arc::new(rhs.into()),
            name: name.into(),
        }
    }

    pub fn operator(&self) -> &PrimitiveOperator {
        &self.operator
    }

    pub fn lhs(&self) -> &Expression {
        &self.lhs
    }

    pub fn rhs(&self) -> &Expression {
        &self.rhs
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
