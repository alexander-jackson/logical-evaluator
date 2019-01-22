use super::*;

#[test]
fn shunting_yard_only_two_and_test() {
    let input = "p&q".to_owned();
    let output = shunting_yard(&input);
    let expected = "pq&";
    assert_eq!(output, expected);
}

#[test]
fn shunting_yard_only_two_or_test() {
    let input = "p|q".to_owned();
    let output = shunting_yard(&input);
    let expected = "pq|";
    assert_eq!(output, expected);
}

#[test]
fn shunting_yard_not_test() {
    let input = "!p".to_owned();
    let output = shunting_yard(&input);
    let expected = "p!";
    assert_eq!(output, expected);
}

#[test]
fn shunting_yard_and_then_or_test() {
    let input = "p&q|r".to_owned();
    let output = shunting_yard(&input);
    let expected = "pq&r|";
    assert_eq!(output, expected);
}

#[test]
fn shunting_yard_or_then_and_test() {
    let input = "p|q&r".to_owned();
    let output = shunting_yard(&input);
    let expected = "pqr&|";
    assert_eq!(output, expected);
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
fn shunting_yard_with_brackets_test() {
    let input = "p&(q|r)".to_owned();
    let output = shunting_yard(&input);
    let expected = "pqr|&";
    assert_eq!(output, expected);
}
