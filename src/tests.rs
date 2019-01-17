use super::*;

#[test]
fn shunting_yard_only_two_and_test() {
    let output = shunting_yard("p&q".to_string());
    let expected = "pq&";
    assert_eq!(output, expected);
}

#[test]
fn shunting_yard_only_two_or_test() {
    let output = shunting_yard("p|q".to_string());
    let expected = "pq|";
    assert_eq!(output, expected);
}

#[test]
fn shunting_yard_and_then_or_test() {
    let output = shunting_yard("p&q|r".to_string());
    let expected = "pq&r|";
    assert_eq!(output, expected);
}

#[test]
fn shunting_yard_or_then_and_test() {
    let output = shunting_yard("p|q&r".to_string());
    let expected = "pqr&|";
    assert_eq!(output, expected);
}

#[test]
fn evaluate_and_operation_test() {
    let output = evaluate("pq&".to_string(), "pq".to_string());
    let expected = true;
    assert_eq!(output, expected);

    let output = evaluate("pq&".to_string(), "p".to_string());
    let expected = false;
    assert_eq!(output, expected);

    let output = evaluate("pq&".to_string(), "q".to_string());
    let expected = false;
    assert_eq!(output, expected);

    let output = evaluate("pq&".to_string(), "".to_string());
    let expected = false;
    assert_eq!(output, expected);
}

#[test]
fn evaluate_or_operation_test() {
    let output = evaluate("pq|".to_string(), "pq".to_string());
    let expected = true;
    assert_eq!(output, expected);

    let output = evaluate("pq|".to_string(), "p".to_string());
    let expected = true;
    assert_eq!(output, expected);

    let output = evaluate("pq|".to_string(), "q".to_string());
    let expected = true;
    assert_eq!(output, expected);

    let output = evaluate("pq|".to_string(), "".to_string());
    let expected = false;
    assert_eq!(output, expected);
}

#[test]
fn shunting_yard_with_brackets_test() {
    let output = shunting_yard("p&(q|r)".to_string());
    let expected = "pqr|&";
    assert_eq!(output, expected);
}
