use std::collections::HashMap;

#[derive(Debug)]
pub enum Expression {
    Binary {
        left: Box<Expression>,
        op: char,
        right: Box<Expression>,
    },
    Unary {
        op: char,
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
            Binary { left, op, right } => evaluate_binary_expr(valuations, left, op, right),
            Unary { op, expr } => evaluate_unary_expr(valuations, op, expr),
            Identifier { value } => evaluate_identifier(valuations, value),
            Enclosed { inner } => inner.evaluate(valuations),
            True => true,
            False => false,
        }
    }

    pub fn get_variables(&self) -> Vec<&str> {
        use self::Expression::*;

        let mut variables = Vec::new();

        match self {
            Binary { left, right, .. } => {
                variables.extend_from_slice(&left.get_variables());
                variables.extend_from_slice(&right.get_variables());
            }
            Unary { expr, .. } => variables.extend_from_slice(&expr.get_variables()),
            Identifier { value } => variables.push(&value),
            Enclosed { inner } => variables.extend_from_slice(&inner.get_variables()),
            _ => (),
        };

        variables
    }
}

fn evaluate_binary_expr(
    valuations: &HashMap<&str, bool>,
    left: &Box<Expression>,
    op: &char,
    right: &Box<Expression>,
) -> bool {
    let lhs = left.evaluate(valuations);
    let rhs = right.evaluate(valuations);

    match op {
        '&' => lhs & rhs,
        '|' => lhs | rhs,
        _ => unreachable!(),
    }
}

fn evaluate_unary_expr(
    valuations: &HashMap<&str, bool>,
    op: &char,
    expr: &Box<Expression>,
) -> bool {
    let value = expr.evaluate(valuations);

    match op {
        '!' => !value,
        _ => unreachable!(),
    }
}

fn evaluate_identifier(valuations: &HashMap<&str, bool>, ident: &str) -> bool {
    let value = valuations.get(ident);

    if let Some(value) = value {
        *value
    } else {
        false
    }
}
