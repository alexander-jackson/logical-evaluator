# Logical Evaluator

Logical Evaluator (le) is a Rust tool for evaluating logical expressions in
different ways.

## Installation

You can install LE by cloning the Git repository, and then using Rust's package
manager Cargo to build and install the executable.

```
git clone https://github.com/alexander-jackson/logical-evaluator.git
cd logical-evaluator/
cargo install --path .
```

## Usage

While in the `logical-evaluator` repository, you can type the following to get
documentation.

```
le -h

Evaluates logical expressions and displays truth tables

USAGE:
    le [FLAGS] [OPTIONS] --formula <WFF>

FLAGS:
    -h, --help           Prints help information
        --solve          Finds the first instance where the formula evaluates to true
        --truth_table    Informs the program to display a truth table for the WFF
    -V, --version        Prints version information

OPTIONS:
        --entails <WFF>            Sets the formula to compare to for entailment
        --equals <WFF>             Sets the formula to compare to for equality
    -f, --formula <WFF>            Sets the formula to act on
        --valuation <VARIABLES>    Sets the variables that are true in the formula
```

## Tests

Tests can be run using the `cargo test` command to ensure all tests work on your system.

## Contributing

Pull requests are welcome if there are features you would like to add. If
requests are large additions, please open an issue first to discuss.

Please also update tests where appropriate.
