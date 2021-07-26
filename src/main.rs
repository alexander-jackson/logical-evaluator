#[macro_use]
extern crate lalrpop_util;

use std::collections::HashMap;

use colored::*;

lalrpop_mod!(pub parser);

pub mod ast;
pub mod lexer;

fn generate_valuation(set_variables: &str) -> HashMap<&str, bool> {
    set_variables.split(' ').map(|var| (var, true)).collect()
}

fn generate_truth_table(expression: &ast::Expression) {
    // Generate the ast
    let variables = expression.get_variables();

    // Generate the initial HashMap
    let mut valuation = HashMap::new();
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
            match valuation.get(c) {
                Some(x) => match x {
                    true => print!("{}", "T\t".green()),
                    false => print!("{}", "F\t".red()),
                },
                None => panic!("Variable didn't exist in the HashMap"),
            };
        }

        if expression.evaluate(&valuation) {
            println!("{}", "T".green())
        } else {
            println!("{}", "F".red())
        }
    }

    println!();
}

fn solve_satisfiability(expression: &ast::Expression) -> Option<HashMap<&str, bool>> {
    let mut valuation: HashMap<&str, bool> = HashMap::new();

    // Find the variables in the expression
    let variables = expression.get_variables();
    let iterations = 2 << (variables.len() - 1);

    for bitmask in 0..iterations {
        // Iterate the variables
        for (i, var) in variables.iter().enumerate() {
            valuation.insert(*var, bitmask & (1 << i) > 0);
        }

        if expression.evaluate(&valuation) {
            return Some(valuation);
        }
    }

    None
}

fn check_entailment(f_ast: &ast::Expression, e_ast: &ast::Expression) -> bool {
    let f_variables: Vec<&str> = f_ast.get_variables();

    let iterations = 2 << (f_variables.len() - 1);
    let mut valuation = HashMap::new();

    for bitmask in 0..iterations {
        // Iterate the variables
        for (i, var) in f_variables.iter().enumerate() {
            valuation.insert(*var, bitmask & (1 << i) > 0);
        }

        let f_value = f_ast.evaluate(&valuation);
        let e_value = e_ast.evaluate(&valuation);

        if f_value && !e_value {
            return false;
        }
    }

    true
}

fn check_equivalence(f_ast: &ast::Expression, e_ast: &ast::Expression) -> bool {
    let f_variables: Vec<&str> = f_ast.get_variables();
    let e_variables: Vec<&str> = e_ast.get_variables();

    if f_variables != e_variables {
        return false;
    }

    let iterations = 2 << (f_variables.len() - 1);
    let mut valuation = HashMap::new();

    for bitmask in 0..iterations {
        // Iterate the variables
        for (i, var) in f_variables.iter().enumerate() {
            valuation.insert(*var, bitmask & (1 << i) > 0);
        }

        let f_value = f_ast.evaluate(&valuation);
        let e_value = e_ast.evaluate(&valuation);

        if f_value != e_value {
            return false;
        }
    }

    true
}

struct Args {
    formula: Option<String>,
    valuation: Option<String>,
    entailment: Option<String>,
    equality: Option<String>,
    truth_table: bool,
    sat_solve: bool,
}

fn parse_args() -> Result<Args, pico_args::Error> {
    let mut args = pico_args::Arguments::from_env();

    let args = Args {
        formula: args.opt_value_from_str("--formula")?,
        valuation: args.opt_value_from_str("--valuation")?,
        entailment: args.opt_value_from_str("--entailment")?,
        equality: args.opt_value_from_str("--equality")?,
        truth_table: args.contains("--truth-table"),
        sat_solve: args.contains("--solve"),
    };

    Ok(args)
}

fn main() {
    let args = parse_args().expect("Failed to parse arguments");
    let formula = args.formula.expect("Argument required for --formula");

    let lexer = lexer::Lexer::new(&formula);
    let parser = parser::ExprParser::new();
    let ast = parser.parse(lexer).unwrap();

    if args.truth_table {
        generate_truth_table(&ast);
    }

    if args.sat_solve {
        if let Some(solution) = solve_satisfiability(&ast) {
            println!("\nSAT Solution: ");
            for (atom, value) in solution {
                println!("{} - {}", atom, if value { "T".green() } else { "F".red() });
            }
        } else {
            println!("No solution was found for this equation.");
        }
    }

    if let Some(valuation) = args.valuation {
        let map = generate_valuation(&valuation);
        let value = ast.evaluate(&map);

        print!("Result of evaluation: ");

        if value {
            println!("{}", "T".green());
        } else {
            println!("{}", "F".red());
        }
    }

    if let Some(entailment) = args.entailment {
        let lexer = lexer::Lexer::new(&entailment);
        let entailment_ast = parser::ExprParser::new().parse(lexer).unwrap();

        let entails = check_entailment(&ast, &entailment_ast);

        println!(
            "{}",
            if entails {
                format!("{} entails {}", &formula, &entailment)
            } else {
                format!("{} does not entail {}", &formula, &entailment)
            }
        );
    }

    if let Some(equality) = args.equality {
        let lexer = lexer::Lexer::new(&equality);
        let equality_ast = parser::ExprParser::new().parse(lexer).unwrap();

        let equal = check_equivalence(&ast, &equality_ast);

        println!(
            "{}",
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
