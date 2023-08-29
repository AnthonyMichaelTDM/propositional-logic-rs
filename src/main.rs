use propositional_logic_rs::prelude::*;

fn main() {
    // example
    let compound_proposition = |p, q, r| -> bool { iff(q, (p && !q) || (!p && q)) && r };

    truth_table!((p, q, r) => compound_proposition);
}
