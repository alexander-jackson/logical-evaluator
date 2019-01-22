use std::env;
use std::collections::HashMap;

use colored::*;
use clap::App;

fn shunting_yard(expression: &String) -> String {
    let mut output: Vec<char> = Vec::new();
    let mut stack: Vec<char> = Vec::new();

    let precedence: HashMap<char, i32> = [
        ('>', 0),
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

    return output.into_iter().collect();
}

fn get_value(atom: char, map: &HashMap<char, bool>) -> bool {
    if atom == 'T' {
        return true;
    }
    else if atom == 'F' {
        return false;
    }

    return match map.get(&atom) {
        Some(_v) => *_v,
        None => false
    };
}

fn generate_valuation(set_variables: &String) -> HashMap<char, bool> {
    let mut valuation: HashMap<char, bool> = HashMap::new();

    for c in set_variables.chars() {
        valuation.insert(c, true);
    }

    valuation.insert('T', true);

    return valuation;
}

fn evaluate_operator(op: &char, stack: &mut Vec<char>, valuation: &HashMap<char, bool>) -> bool {
    let first: bool = get_value(stack.pop().unwrap(), &valuation);

    let second: bool = match *op {
        '!' => false,
        _ => get_value(stack.pop().unwrap(), &valuation)
    };

    return match op {
        '!' => !first,
        '&' => first & second,
        '|' => first | second,
        '>' => first | !second,
        _ => panic!("Unexpected operation: {}", op)
    };
}

fn evaluate(ast: &String, valuation: &HashMap<char, bool>) -> bool {
    let mut stack: Vec<char> = Vec::new();

    for c in ast.chars() {
        if c.is_alphabetic() {
            stack.push(c);
        } else {
            let val: bool = evaluate_operator(&c, &mut stack, &valuation);

            stack.push(
                match val {
                    true => 'T',
                    false => 'F'
                }
            );
        }
    }

    return match stack.pop().unwrap() {
        'T' => true,
        _ => false
    }
}

fn generate_truth_table(expression: &String) {
    // Generate the ast
    let ast: String = shunting_yard(&expression);

    // Find the variables in the expression
    let mut variables: Vec<char> = Vec::new();

    for c in ast.chars() {
        if c.is_alphabetic() && !variables.contains(&c) {
            variables.push(c);
        }
    }

    // Generate the initial HashMap
    let mut valuation: HashMap<char, bool> = HashMap::new();
    let iterations = 2 << (variables.len() - 1);

    for c in &variables {
        valuation.insert(*c, false);

        print!("{}\t", c);
    }

    println!("result:");

    for bitmask in 0..iterations {
        // Iterate the variables
        for i in 0..variables.len() {
            if bitmask & (1 << i) > 0 {
                valuation.insert(variables[i], true);
            } else {
                valuation.insert(variables[i], false);
            }
        }

        for c in &variables {
            match valuation.get(&c) {
                Some(x) => match x {
                    true => print!("{}", "T\t".green()),
                    false => print!("{}", "F\t".red()),
                },
                None => panic!("Variable didn't exist in the HashMap")
            };
        }

        match evaluate(&ast, &valuation) {
            true => println!("{}", "T".green()),
            false => println!("{}", "F".red())
        }
    }

    println!("");
}

fn main() {
    let yaml = clap::load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let formula = matches.value_of("formula").unwrap().to_string();
    let truth_table = matches.is_present("truth_table");
    let valuation = matches.value_of("valuation").unwrap_or("").to_string();

    if truth_table {
        generate_truth_table(&formula);
    }

    if &valuation != "" {
        let ast = shunting_yard(&formula);
        let map = generate_valuation(&valuation);

        let value = evaluate(&ast, &map);

        print!("Result of evaluation: ");

        match value {
            true => println!("{}", "T".green()),
            false => println!("{}", "F".red())
        };
    }
}

#[cfg(test)]
mod tests;
