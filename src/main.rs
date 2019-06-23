use std::collections::HashMap;

use colored::*;
use clap::App;
use regex::Regex;

fn shunting_yard(expression: &str) -> String {
    let mut output: Vec<char> = Vec::new();
    let mut stack: Vec<char> = Vec::new();

    let space_re = Regex::new(r"[ ]*").unwrap();

    let no_spaces = space_re.replace_all(expression, "");
    let finished = no_spaces.replace("=>", ">");

    let precedence: HashMap<char, i32> = [
        ('>', 0),
        ('|', 1),
        ('&', 2),
        ('!', 3),
        ('(', 4)
    ].iter().cloned().collect();

    for c in finished.chars() {
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

                if stack.is_empty() {
                    break;
                }

                top = stack.pop().unwrap();
            }
        }
        else if c == ' ' {
            continue;
        }
        else {
            while !stack.is_empty() && precedence[&stack[stack.len() - 1]] > precedence[&c] && stack[stack.len() - 1] != '(' {
                output.push(stack.pop().unwrap());
            }

            stack.push(c);
        }
    }

    while !stack.is_empty() {
        output.push(stack.pop().unwrap());
    }

    output.into_iter().collect()
}

fn get_variables(input: &str) -> Vec<char> {
    let mut variables: Vec<char> = Vec::new();

    for c in input.chars() {
        if c.is_alphabetic() && !variables.contains(&c) && c != 'T' && c != 'F' {
            variables.push(c);
        }
    }

    variables
}

fn get_value(atom: char, map: &HashMap<char, bool>) -> bool {
    if atom == 'T' {
        return true;
    }
    else if atom == 'F' {
        return false;
    }

    match map.get(&atom) {
        Some(_v) => *_v,
        None => false
    }
}

fn generate_valuation(set_variables: &str) -> HashMap<char, bool> {
    let mut valuation: HashMap<char, bool> = HashMap::new();

    for c in set_variables.chars() {
        if c != 'T' && c != 'F' {
            valuation.insert(c, true);
        }
    }

    valuation
}

fn evaluate_operator(op: char, stack: &mut Vec<char>, valuation: &HashMap<char, bool>) -> bool {
    let first: bool = get_value(stack.pop().unwrap(), &valuation);

    let second: bool = match op {
        '!' => false,
        _ => get_value(stack.pop().unwrap(), &valuation)
    };

    match op {
        '!' => !first,
        '&' => first & second,
        '|' => first | second,
        '>' => first | !second,
        _ => panic!("Unexpected operation: {}", op)
    }
}

fn evaluate(ast: &str, valuation: &HashMap<char, bool>) -> bool {
    let mut stack: Vec<char> = Vec::new();

    for c in ast.chars() {
        if c.is_alphabetic() {
            stack.push(c);
        } else {
            let val: bool = evaluate_operator(c, &mut stack, &valuation);

            stack.push(
                if val {
                    'T'
                } else {
                    'F'
                }
            );
        }
    }

    match stack.pop().unwrap() {
        'T' => true,
        _ => false
    }
}

fn generate_truth_table(expression: &str) {
    // Generate the ast
    let ast: String = shunting_yard(&expression);
    let variables: Vec<char> = get_variables(&ast);

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
        for (i, var) in variables.iter().enumerate() {
            valuation.insert(*var, bitmask & (1 << i) > 0);
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

        if evaluate(&ast, &valuation) {
            println!("{}", "T".green())
        } else {
            println!("{}", "F".red())
        }
    }

    println!();
}

fn solve_satisfiability(formula: &str) -> (HashMap<char, bool>, bool) {
    let mut valuation: HashMap<char, bool> = HashMap::new();

    // Find the variables in the expression
    let ast = shunting_yard(&formula);
    let variables: Vec<char> = get_variables(&ast);

    let iterations = 2 << (variables.len() - 1);

    for bitmask in 0..iterations {
        // Iterate the variables
        for (i, var) in variables.iter().enumerate() {
            valuation.insert(*var, bitmask & (1 << i) > 0);
        }

        if evaluate(&ast, &valuation) {
            return (valuation, true)
        } else {
            continue
        }
    }

    (valuation, false)
}

fn check_entailment(f_ast: &str, e_ast: &str) -> bool {
    let f_variables: Vec<char> = get_variables(&f_ast);

    let iterations = 2 << (f_variables.len() - 1);
    let mut valuation: HashMap<char, bool> = HashMap::new();

    for bitmask in 0..iterations {
        // Iterate the variables
        for (i, var) in f_variables.iter().enumerate() {
            valuation.insert(*var, bitmask & (1 << i) > 0);
        }

        let f_value = evaluate(&f_ast, &valuation);
        let e_value = evaluate(&e_ast, &valuation);

        if f_value && !e_value {
            return false;
        }
    }

    true
}

fn check_equivalence(f_ast: &str, e_ast: &str) -> bool {
    let f_variables: Vec<char> = get_variables(&f_ast);
    let e_variables: Vec<char> = get_variables(&e_ast);

    if f_variables != e_variables {
        return false;
    }

    let iterations = 2 << (f_variables.len() - 1);
    let mut valuation: HashMap<char, bool> = HashMap::new();

    for bitmask in 0..iterations {
        // Iterate the variables
        for (i, var) in f_variables.iter().enumerate() {
            valuation.insert(*var, bitmask & (1 << i) > 0);
        }

        let f_value = evaluate(&f_ast, &valuation);
        let e_value = evaluate(&e_ast, &valuation);

        if f_value != e_value {
            return false;
        }
    }

    true
}

fn main() {
    let yaml = clap::load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let formula = matches.value_of("formula").unwrap().to_string();
    let valuation = matches.value_of("valuation").unwrap_or("").to_string();
    let entailment = matches.value_of("entails").unwrap_or("").to_string();
    let equality = matches.value_of("equals").unwrap_or("").to_string();

    let truth_table = matches.is_present("truth_table");
    let sat_solve = matches.is_present("solve");

    if truth_table {
        generate_truth_table(&formula);
    }

    if sat_solve {
        let solution = solve_satisfiability(&formula);

        if solution.1 {
            println!("\nSAT Solution: ");
            for (atom, value) in solution.0 {
                println!("{} - {}", atom,
                    if value {
                        "T".green()
                    } else {
                        "F".red()
                    }
                );
            }
        } else {
            println!("No solution was found for this equation.");
        }
    }

    if &valuation != "" {
        let ast = shunting_yard(&formula);
        let map = generate_valuation(&valuation);

        let value = evaluate(&ast, &map);

        print!("Result of evaluation: ");

        if value {
            println!("{}", "T".green());
        } else {
            println!("{}", "F".red());
        }
    }

    if &entailment != "" {
        let f_ast = shunting_yard(&formula);
        let e_ast = shunting_yard(&entailment);

        let entails = check_entailment(&f_ast, &e_ast);

        println!("{}",
            if entails {
                format!("{} entails {}", &formula, &entailment)
            } else {
                format!("{} does not entail {}", &formula, &entailment)
            }
        );
    }

    if &equality != "" {
        let f_ast = shunting_yard(&formula);
        let e_ast = shunting_yard(&equality);

        let equal = check_equivalence(&f_ast, &e_ast);

        println!("{}",
            if equal {
                format!("{} equals {}", &formula, &equality)
            } else {
                format!("{} does not equal {}", &formula, &equality)
            }
        );
    }
}

#[cfg(test)]
mod tests;
