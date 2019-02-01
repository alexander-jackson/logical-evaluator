use std::collections::HashMap;

use colored::*;
use clap::App;
use regex::Regex;

fn shunting_yard(expression: &String) -> String {
    let mut output: Vec<char> = Vec::new();
    let mut stack: Vec<char> = Vec::new();

    let space_re = Regex::new(r"[ ]*").unwrap();
    let implies_re = Regex::new(r"=>").unwrap();

    let no_spaces = space_re.replace_all(expression, "");
    let finished = implies_re.replace_all(&no_spaces, ">");

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

                if stack.len() == 0 {
                    break;
                }

                top = stack.pop().unwrap();
            }
        }
        else if c == ' ' {
            continue;
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

fn get_variables(input: &String) -> Vec<char> {
    let mut variables: Vec<char> = Vec::new();

    for c in input.chars() {
        if c.is_alphabetic() && !variables.contains(&c) && c != 'T' && c != 'F' {
            variables.push(c);
        }
    }

    return variables;
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
        if c != 'T' && c != 'F' {
            valuation.insert(c, true);
        }
    }

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

fn solve_satisfiability(formula: &String) -> (HashMap<char, bool>, bool) {
    let mut valuation: HashMap<char, bool> = HashMap::new();

    // Find the variables in the expression
    let ast = shunting_yard(&formula);
    let variables: Vec<char> = get_variables(&ast);

    let iterations = 2 << (variables.len() - 1);

    for bitmask in 0..iterations {
        // Iterate the variables
        for i in 0..variables.len() {
            if bitmask & (1 << i) > 0 {
                valuation.insert(variables[i], true);
            } else {
                valuation.insert(variables[i], false);
            }
        }

        match evaluate(&ast, &valuation) {
            true => return (valuation, true),
            false => continue
        }
    }

    return (valuation, false);
}

fn check_entailment(f_ast: &String, e_ast: &String) -> bool {
    let f_variables: Vec<char> = get_variables(&f_ast);

    let iterations = 2 << (f_variables.len() - 1);
    let mut valuation: HashMap<char, bool> = HashMap::new();

    for bitmask in 0..iterations {
        // Iterate the variables
        for i in 0..f_variables.len() {
            if bitmask & (1 << i) > 0 {
                valuation.insert(f_variables[i], true);
            } else {
                valuation.insert(f_variables[i], false);
            }
        }

        let f_value = evaluate(&f_ast, &valuation);
        let e_value = evaluate(&e_ast, &valuation);

        if f_value && !e_value {
            return false;
        }
    }

    return true;
}

fn check_equivalence(f_ast: &String, e_ast: &String) -> bool {
    let f_variables: Vec<char> = get_variables(&f_ast);
    let e_variables: Vec<char> = get_variables(&e_ast);

    if f_variables != e_variables {
        return false;
    }

    let iterations = 2 << (f_variables.len() - 1);
    let mut valuation: HashMap<char, bool> = HashMap::new();

    for bitmask in 0..iterations {
        // Iterate the variables
        for i in 0..f_variables.len() {
            if bitmask & (1 << i) > 0 {
                valuation.insert(f_variables[i], true);
            } else {
                valuation.insert(f_variables[i], false);
            }
        }

        let f_value = evaluate(&f_ast, &valuation);
        let e_value = evaluate(&e_ast, &valuation);

        if f_value != e_value {
            return false;
        }
    }

    return true;
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
                    match value {
                        true => "T".green(),
                        false => "F".red()
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

        match value {
            true => println!("{}", "T".green()),
            false => println!("{}", "F".red())
        };
    }

    if &entailment != "" {
        let f_ast = shunting_yard(&formula);
        let e_ast = shunting_yard(&entailment);

        let entails = check_entailment(&f_ast, &e_ast);

        println!("{}", match entails {
            true => format!("{} entails {}", &formula, &entailment),
            false => format!("{} does not entail {}", &formula, &entailment),
        });
    }

    if &equality != "" {
        let f_ast = shunting_yard(&formula);
        let e_ast = shunting_yard(&equality);

        let equal = check_equivalence(&f_ast, &e_ast);

        println!("{}", match equal {
            true => format!("{} equals {}", &formula, &entailment),
            false => format!("{} does not equal {}", &formula, &entailment),
        });
    }
}

#[cfg(test)]
mod tests;
