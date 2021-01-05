use super::expression::Expression;
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub struct AtomicLoad {
    pointer: Arc<Expression>,
    name: String,
}

impl AtomicLoad {
    pub fn new(pointer: impl Into<Expression>, name: impl Into<String>) -> Self {
        Self {
            pointer: pointer.into().into(),
            name: name.into(),
        }
    }

    pub fn pointer(&self) -> &Expression {
        &self.pointer
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
