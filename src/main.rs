use propositional_logic_rs::prelude::*;

fn main() {
    // example
    let compound_proposition = |p, q, r| -> bool { iff(q, (p && !q) || (!p && q)) && r };

    print_truth_table!(|p, q, r| => compound_proposition);

    print_truth_table!(inline_compound_proposition => |p, q, r| {
        iff(q, (p && !q) || (!p && q)) && r
    });
}
