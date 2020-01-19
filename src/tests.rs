use super::*;

macro_rules! shunting_yard {
    ($($name:ident: $input:expr, $expected:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let input: &str = $input;
                let output: &str = &shunting_yard(input);
                let expected: &str = $expected;
                assert_eq!(output, expected);
            }
        )*
    }
}

shunting_yard! {
    logical_and: "p&q", "pq&",
    logical_or: "p|q", "pq|",
    logical_not: "!p", "p!",
    implication: "p>q", "pq>",
    and_then_or: "p&q|r", "pq&r|",
    or_then_and: "p|q&r", "pqr&|",
    ignore_spaces: "p & q", "pq&",
    long_implication: "p = > q", "pq>",
}

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

    assert!(evaluate(&ast, &output.0));
}

#[test]
fn simple_sat_solve_all_false_test() {
    let input = "!(p|q)".to_owned();
    let ast = shunting_yard(&input);
    let output = solve_satisfiability(&input);

    assert!(evaluate(&ast, &output.0));
}

#[test]
fn sat_solve_impossible_test() {
    let input = "!p&p".to_owned();
    let output = solve_satisfiability(&input);

    assert!(!output.1);
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
