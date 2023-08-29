# Propositional Logic

A rust library for generating the truth table of any single compound proposition.

only has one dependency: [cli_table](https://crates.io/crates/cli_table)

## Usage

```rust
use propositional_logic::prelude::*;

let compound_proposition = |p, q, r| -> bool { iff(q, (p && !q) || (!p && q)) && r };

print_truth_table!(|p, q, r| => compound_proposition);

println!();

// or inline
print_truth_table!(inline_compound_proposition => |p, q, r| {
   iff(q, (p && !q) || (!p && q)) && r
})
```

Outputs:

```text
+-------+-------+-------+----------------------+
| p     | q     | r     | compound_proposition |
+-------+-------+-------+----------------------+
| true  | true  | true  |                false |
+-------+-------+-------+----------------------+
| true  | true  | false |                false |
+-------+-------+-------+----------------------+
| true  | false | true  |                false |
+-------+-------+-------+----------------------+
| true  | false | false |                false |
+-------+-------+-------+----------------------+
| false | true  | true  |                 true |
+-------+-------+-------+----------------------+
| false | true  | false |                false |
+-------+-------+-------+----------------------+
| false | false | true  |                 true |
+-------+-------+-------+----------------------+
| false | false | false |                false |
+-------+-------+-------+----------------------+

+-------+-------+-------+-----------------------------+
| p     | q     | r     | inline_compound_proposition |
+-------+-------+-------+-----------------------------+
| true  | true  | true  |                       false |
+-------+-------+-------+-----------------------------+
| true  | true  | false |                       false |
+-------+-------+-------+-----------------------------+
| true  | false | true  |                       false |
+-------+-------+-------+-----------------------------+
| true  | false | false |                       false |
+-------+-------+-------+-----------------------------+
| false | true  | true  |                        true |
+-------+-------+-------+-----------------------------+
| false | true  | false |                       false |
+-------+-------+-------+-----------------------------+
| false | false | true  |                        true |
+-------+-------+-------+-----------------------------+
| false | false | false |                       false |
+-------+-------+-------+-----------------------------+
```
