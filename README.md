# Logical Evaluator

Logical Evaluator (LE) is a Rust tool for evaluating logical expressions in
different ways.

## Installation

You can install LE by cloning the Git repository, and then using Rust's package
manager Cargo to build the executable.

```
git clone https://github.com/alexander-jackson/logical-evaluator.git
cd logical-evaluator/
cargo build
```

## Usage

While in the `logical-evaluator` repository, you can type:

```
cargo run -- -h
```

to get the documentation.

A simple test to ensure it build properly is to do
`cargo run -- --formula 'a&b' --truth_table' to get the truth table for A and
B.

## Contributing

Pull requests are welcome if there are features you would like to add. If
requests are large additions, please open an issue first to discuss.

Please also update tests where appropriate.
