use std::env;
use std::collections::HashMap;

fn shunting_yard(expression: String) {
    let mut output: Vec<char> = Vec::new();
    let mut stack: Vec<char> = Vec::new();

    let precedence: HashMap<char, i32> = [
        ('|', 1),
        ('&', 2),
        ('!', 3)
    ].iter().cloned().collect();

    for c in expression.chars() {
        println!("{}", c);
        println!("Alphabetic: {}", c.is_alphabetic());

        if c.is_alphabetic() {
            output.push(c);
        } else {
            while stack.len() > 0 && precedence[&stack[0]] > precedence[&c] {
                output.push(stack.pop().unwrap());
            }

            stack.push(c);
        }
    }

    while stack.len() > 0 {
        output.push(stack.pop().unwrap());
    }

    println!("{:?}", output);
    println!("{:?}", stack);

    let parsed: String = output.into_iter().collect();

    println!("{}", parsed);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("Command line arguments: {:?}", args);

    assert!(args.len() == 3);

    let expression = &args[1];
    let valuation = &args[2];

    println!("Expression: {}", expression);
    println!("Valuation: {}", valuation);

    let ast = shunting_yard(expression.to_string());
}
