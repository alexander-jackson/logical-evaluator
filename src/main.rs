use std::env;
use std::collections::HashMap;

fn shunting_yard(expression: String) -> String {
    let mut output: Vec<char> = Vec::new();
    let mut stack: Vec<char> = Vec::new();

    let precedence: HashMap<char, i32> = [
        ('|', 1),
        ('&', 2),
        ('!', 3),
        ('(', 4)
    ].iter().cloned().collect();

    for c in expression.chars() {
        if c.is_alphabetic() {
            output.push(c);
        }
        else if c == '(' {
            stack.push(c);
        }
        else if c == ')' {
            let mut top = stack.pop().unwrap();

            while top != '(' {
                output.push(top);

                if stack.len() == 0 {
                    break;
                }

                top = stack.pop().unwrap();
            }
        }
        else {
            while stack.len() > 0 && precedence[&stack[stack.len() - 1]] > precedence[&c] && stack[stack.len() - 1] != '(' {
                output.push(stack.pop().unwrap());
            }

            stack.push(c);
        }
    }

    while stack.len() > 0 {
        output.push(stack.pop().unwrap());
    }

    let parsed: String = output.into_iter().collect();

    return parsed;
}

fn get_value(atom: char, map: &HashMap<char, bool>) -> bool {
    return match map.get(&atom) {
        Some(_v) => true,
        None => false
    };
}

fn evaluate(ast: String, valuation: String) -> bool {
    let mut stack: Vec<char> = Vec::new();
    let mut values: HashMap<char, bool> = HashMap::new();

    for c in valuation.chars() {
        values.insert(c, true);
    }

    values.insert('T', true);

    for c in ast.chars() {
        if c.is_alphabetic() {
            stack.push(c);
        } else {
            let op = c;

            let a: bool = get_value(stack.pop().unwrap(), &values);
            let b: bool = get_value(stack.pop().unwrap(), &values);

            let mut val: bool = false;

            if op == '&' {
                val = a & b;
            } else if op == '|' {
                val = a | b;
            }

            if val {
                stack.push('T');
            } else {
                stack.push('F');
            }
        }
    }

    if stack.pop().unwrap() == 'T' {
        return true;
    } else {
        return false;
    }
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
    println!("Parsed: {}", ast);
    println!("Evaluation: {}", evaluate(ast, valuation.to_string()));
}

#[cfg(test)]
mod tests;
