use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, Debug)]
pub enum BinaryOperator {
    And,
    Or,
    Implies,
}

impl BinaryOperator {
    fn evaluate(&self, left: bool, right: bool) -> bool {
        match *self {
            Self::And => left & right,
            Self::Or => left | right,
            Self::Implies => !left | right,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum UnaryOperator {
    Not,
}

impl UnaryOperator {
    fn evaluate(&self, value: bool) -> bool {
        match *self {
            Self::Not => !value,
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    Binary {
        left: Box<Expression>,
        op: BinaryOperator,
        right: Box<Expression>,
    },
    Unary {
        op: UnaryOperator,
        expr: Box<Expression>,
    },
    Enclosed {
        inner: Box<Expression>,
    },
    Identifier {
        value: String,
    },
    True,
    False,
}

impl Expression {
    pub fn evaluate(&self, valuations: &HashMap<&str, bool>) -> bool {
        use self::Expression::*;

        match self {
            Binary { left, op, right } => evaluate_binary_expr(valuations, left, *op, right),
            Unary { op, expr } => evaluate_unary_expr(valuations, *op, expr),
            Identifier { value } => evaluate_identifier(valuations, value),
            Enclosed { inner } => inner.evaluate(valuations),
            True => true,
            False => false,
        }
    }

    pub fn get_variables(&self) -> HashSet<&str> {
        use self::Expression::*;

        let mut variables = HashSet::new();

        match self {
            Binary { left, right, .. } => {
                variables.extend(left.get_variables());
                variables.extend(right.get_variables());
            }
            Unary { expr, .. } => variables.extend(expr.get_variables()),
            Identifier { value } => {
                let _ = variables.insert(&value);
            }
            Enclosed { inner } => variables.extend(inner.get_variables()),
            _ => (),
        };

        variables
    }
}

fn evaluate_binary_expr(
    valuations: &HashMap<&str, bool>,
    left: &Box<Expression>,
    op: BinaryOperator,
    right: &Box<Expression>,
) -> bool {
    let lhs = left.evaluate(valuations);
    let rhs = right.evaluate(valuations);

    op.evaluate(lhs, rhs)
}

fn evaluate_unary_expr(
    valuations: &HashMap<&str, bool>,
    op: UnaryOperator,
    expr: &Box<Expression>,
) -> bool {
    let value = expr.evaluate(valuations);

    op.evaluate(value)
}

fn evaluate_identifier(valuations: &HashMap<&str, bool>, ident: &str) -> bool {
    let value = valuations.get(ident);

    if let Some(value) = value {
        *value
    } else {
        false
    }
}
