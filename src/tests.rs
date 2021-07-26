use super::*;

macro_rules! parser_success {
    ($($name:ident: $input:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let input: &str = $input;
                let lexer = lexer::Lexer::new(input);
                let parser = parser::ExprParser::new();
                let ast = parser.parse(lexer);
                assert!(ast.is_ok());
            }
        )*
    }
}

parser_success! {
    logical_and: "p&q",
    logical_or: "p|q",
    logical_not: "!p",
    implication: "p=>q",
    and_then_or: "p&q|r",
    or_then_and: "p|q&r",
    ignore_spaces: "p & q",
}

/*
#[test]
fn evaluate_and_operation_test() {
    let input = "pq&".to_owned();
    let mut valuation = HashMap::new();

    valuation.insert('p', true);
    valuation.insert('q', true);

    let output = evaluate(&input, &valuation);
    let expected = true;
    assert_eq!(output, expected);

    valuation.insert('p', true);
    valuation.insert('q', false);

    let output = evaluate(&input, &valuation);
    let expected = false;
    assert_eq!(output, expected);

    valuation.insert('p', false);
    valuation.insert('q', true);

    let output = evaluate(&input, &valuation);
    let expected = false;
    assert_eq!(output, expected);

    valuation.insert('p', false);
    valuation.insert('q', false);

    let output = evaluate(&input, &valuation);
    let expected = false;
    assert_eq!(output, expected);
}

#[test]
fn evaluate_or_operation_test() {
    let input = "pq|".to_owned();
    let mut valuation = HashMap::new();

    valuation.insert('p', true);
    valuation.insert('q', true);

    let output = evaluate(&input, &valuation);
    let expected = true;
    assert_eq!(output, expected);

    valuation.insert('p', true);
    valuation.insert('q', false);

    let output = evaluate(&input, &valuation);
    let expected = true;
    assert_eq!(output, expected);

    valuation.insert('p', false);
    valuation.insert('q', true);

    let output = evaluate(&input, &valuation);
    let expected = true;
    assert_eq!(output, expected);

    valuation.insert('p', false);
    valuation.insert('q', false);

    let output = evaluate(&input, &valuation);
    let expected = false;
    assert_eq!(output, expected);
}

#[test]
fn evaluate_not_operation_test() {
    let input = "p!".to_owned();
    let mut valuation = HashMap::new();

    valuation.insert('p', true);

    let output = evaluate(&input, &valuation);
    let expected = false;
    assert_eq!(output, expected);

    valuation.insert('p', false);

    let output = evaluate(&input, &valuation);
    let expected = true;
    assert_eq!(output, expected);
}

#[test]
fn evaluate_implication_operation_test() {
    let input = "pq>".to_owned();
    let mut valuation = HashMap::new();

    valuation.insert('p', true);
    valuation.insert('q', true);

    let output = evaluate(&input, &valuation);
    let expected = true;
    assert_eq!(output, expected);

    valuation.insert('p', true);
    valuation.insert('q', false);

    let output = evaluate(&input, &valuation);
    let expected = false;
    assert_eq!(output, expected);

    valuation.insert('p', false);
    valuation.insert('q', true);

    let output = evaluate(&input, &valuation);
    let expected = true;
    assert_eq!(output, expected);

    valuation.insert('p', false);
    valuation.insert('q', false);

    let output = evaluate(&input, &valuation);
    let expected = true;
    assert_eq!(output, expected);
}

#[test]
fn get_variables_test() {
    let input = "TF".to_owned();
    let output = get_variables(&input);

    assert_eq!(output.len(), 0);
}

#[test]
fn generate_valuation_test() {
    let input = "TF".to_owned();
    let output = generate_valuation(&input);

    assert_eq!(output.len(), 0);
}

#[test]
fn shunting_yard_with_brackets_test() {
    let input = "p&(q|r)".to_owned();
    let output = shunting_yard(&input);
    let expected = "pqr|&";
    assert_eq!(output, expected);
}

#[test]
fn simple_sat_solve_all_true_test() {
    let input = "p&q".to_owned();
    let ast = shunting_yard(&input);
    let output = solve_satisfiability(&input);

    assert!(evaluate(&ast, &output.unwrap()));
}

#[test]
fn simple_sat_solve_all_false_test() {
    let input = "!(p|q)".to_owned();
    let ast = shunting_yard(&input);
    let output = solve_satisfiability(&input);

    assert!(evaluate(&ast, &output.unwrap()));
}

#[test]
fn sat_solve_impossible_test() {
    let input = "!p&p".to_owned();
    let output = solve_satisfiability(&input);

    assert!(output.is_none());
}

#[test]
fn simple_entailment_success_test() {
    let f_input = "pq&".to_owned();
    let e_input = "pq|".to_owned();
    let output = check_entailment(&f_input, &e_input);

    assert!(output);
}

#[test]
fn simple_entailment_fail_test() {
    let f_input = "pq&".to_owned();
    let e_input = "pq|!".to_owned();
    let output = check_entailment(&f_input, &e_input);

    assert!(!output);
}

#[test]
fn simple_equivalence_success_test() {
    let f_input = "pq&".to_owned();
    let e_input = "pq&".to_owned();
    let output = check_entailment(&f_input, &e_input);

    assert!(output);
}

#[test]
fn simple_equivalence_fail_test() {
    let f_input = "pq&".to_owned();
    let e_input = "pq|".to_owned();
    let output = check_entailment(&f_input, &e_input);

    assert!(output);
}
*/
