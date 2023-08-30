# Propositional Logic

![crates.io](https://img.shields.io/crates/v/propositional_logic.svg)
[![CI](https://github.com/AnthonyMichaelTDM/propositional-logic-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/AnthonyMichaelTDM/propositional-logic-rs/actions/workflows/rust.yml)

A rust library for generating the truth table of any single compound proposition.

only has one dependency: [cli_table](https://crates.io/crates/cli_table)

## Usage

### truth_table! macro

this macro only creates and returns the truth table.

```rust
use propositional_logic::prelude::*;
use cli_table::{print_stdout, TableStruct};

let compound_proposition = |p, q, r| -> bool { iff(q, (p && !q) || (!p && q)) && r };

// giving it a function (any function with boolean input and output works, technically)
let table: TableStruct = truth_table!(|p, q, r| => compound_proposition);

// giving it an inline proposition (similar to a closure)
let table: TableStruct = truth_table!(|p, q, r| => inline_compound_proposition -> {
   iff(q, (p && !q) || (!p && q)) && r
});

// giving it both (you can give it as many as you want)
let table: TableStruct = truth_table!(|p, q, r| => {
   compound_proposition,
   inline_compound_proposition -> {
      iff(q, (p && !q) || (!p && q)) && r
   }
});

assert!(print_stdout(table).is_ok());
```

Outputs:

```text
+-------+-------+-------+----------------------+-----------------------------+
| p     | q     | r     | compound_proposition | inline_compound_proposition |
+-------+-------+-------+----------------------+-----------------------------+
| true  | true  | true  |                false |                       false |
+-------+-------+-------+----------------------+-----------------------------+
| true  | true  | false |                false |                       false |
+-------+-------+-------+----------------------+-----------------------------+
| true  | false | true  |                false |                       false |
+-------+-------+-------+----------------------+-----------------------------+
| true  | false | false |                false |                       false |
+-------+-------+-------+----------------------+-----------------------------+
| false | true  | true  |                 true |                        true |
+-------+-------+-------+----------------------+-----------------------------+
| false | true  | false |                false |                       false |
+-------+-------+-------+----------------------+-----------------------------+
| false | false | true  |                 true |                        true |
+-------+-------+-------+----------------------+-----------------------------+
| false | false | false |                false |                       false |
+-------+-------+-------+----------------------+-----------------------------+
```

### print_truth_table! macro

```rust
use propositional_logic::prelude::*;

let compound_proposition = |p, q| -> bool { iff(q, p) };

// giving it a function (any function with boolean input and output works, technically)
print_truth_table!(|p, q| => compound_proposition);
println!();
// giving it an inline proposition (similar to a closure)
print_truth_table!(|p| => inline_compound_proposition -> {
   not(p)
});
println!();
// giving it both (you can give it as many as you want, but every item must have a comma at the end (even the last one))
print_truth_table!(|p,q| => {
   compound_proposition,
   inline_compound_proposition -> {
      not(p)
   },
});
```

Output:

```text
+-------+-------+----------------------+
| p     | q     | compound_proposition |
+-------+-------+----------------------+
| true  | true  |                 true |
+-------+-------+----------------------+
| true  | false |                false |
+-------+-------+----------------------+
| false | true  |                false |
+-------+-------+----------------------+
| false | false |                 true |
+-------+-------+----------------------+

+-------+-----------------------------+
| p     | inline_compound_proposition |
+-------+-----------------------------+
| true  |                       false |
+-------+-----------------------------+
| false |                        true |
+-------+-----------------------------+

+-------+-------+----------------------+-----------------------------+
| p     | q     | compound_proposition | inline_compound_proposition |
+-------+-------+----------------------+-----------------------------+
| true  | true  |                 true |                       false |
+-------+-------+----------------------+-----------------------------+
| true  | false |                false |                       false |
+-------+-------+----------------------+-----------------------------+
| false | true  |                false |                        true |
+-------+-------+----------------------+-----------------------------+
| false | false |                 true |                        true |
+-------+-------+----------------------+-----------------------------+
```
