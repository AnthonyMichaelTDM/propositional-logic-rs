/// Returns a vector of all possible truth values for a given number of atomic propositions.
pub fn possible_truth_values(n: usize) -> Vec<Vec<bool>> {
    let mut truth_values = Vec::new();

    for i in (0..2usize.pow(n as u32)).rev() {
        let mut truth_value = Vec::new();

        for j in (0..n).rev() {
            truth_value.push((i >> j) % 2 == 1);
        }

        truth_values.push(truth_value);
    }

    truth_values
}

/// A macro to print a truth table for a given proposition.
///
/// # Example
///
/// ```
/// use propositional_logic_rs::prelude::*;
///
/// let compound_proposition = |p, q, r| -> bool { iff(q, (p && !q) || (!p && q)) && r };
///
/// print_truth_table!(|p, q, r| => compound_proposition);
///
/// println!();
///
/// // or inline
/// print_truth_table!(inline_compound_proposition => |p, q, r| {
///    iff(q, (p && !q) || (!p && q)) && r
/// })
/// ```
///
/// # Output
///
/// ```text
/// +-------+-------+-------+----------------------+
/// | p     | q     | r     | compound_proposition |
/// +-------+-------+-------+----------------------+
/// | true  | true  | true  |                false |
/// +-------+-------+-------+----------------------+
/// | true  | true  | false |                false |
/// +-------+-------+-------+----------------------+
/// | true  | false | true  |                false |
/// +-------+-------+-------+----------------------+
/// | true  | false | false |                false |
/// +-------+-------+-------+----------------------+
/// | false | true  | true  |                 true |
/// +-------+-------+-------+----------------------+
/// | false | true  | false |                false |
/// +-------+-------+-------+----------------------+
/// | false | false | true  |                 true |
/// +-------+-------+-------+----------------------+
/// | false | false | false |                false |
/// +-------+-------+-------+----------------------+
///
/// +-------+-------+-------+-----------------------------+
/// | p     | q     | r     | inline_compound_proposition |
/// +-------+-------+-------+-----------------------------+
/// | true  | true  | true  |                       false |
/// +-------+-------+-------+-----------------------------+
/// | true  | true  | false |                       false |
/// +-------+-------+-------+-----------------------------+
/// | true  | false | true  |                       false |
/// +-------+-------+-------+-----------------------------+
/// | true  | false | false |                       false |
/// +-------+-------+-------+-----------------------------+
/// | false | true  | true  |                        true |
/// +-------+-------+-------+-----------------------------+
/// | false | true  | false |                       false |
/// +-------+-------+-------+-----------------------------+
/// | false | false | true  |                        true |
/// +-------+-------+-------+-----------------------------+
/// | false | false | false |                       false |
/// +-------+-------+-------+-----------------------------+
/// ```
#[macro_export]
macro_rules! print_truth_table {
    ($prop_name:ident => |$($args:ident),*|  {$($body:tt)*}) => {
        {
            let $prop_name = | $($args: bool),* | -> bool {
                $($body)*
            };
            print_truth_table!(|$($args),*| => $prop_name);
        }
    };

    (|$($atomic:ident),*| => $proposition:expr) => {
        {
            use std::convert::identity;

            use cli_table::{format::Justify, print_stdout, Cell, CellStruct, Style, Table};

            // DATA
            let mut number_of_atomic_propositions = 0;
            $(
                number_of_atomic_propositions += 1;
                let _ = identity(stringify!($atomic)); // a no-op to make sure the compiler knows that $atomic is used
            )*
            let atomic_rows: Vec<Vec<bool>> = possible_truth_values(number_of_atomic_propositions);

            // TABLE
            let table = atomic_rows.iter().cloned().map(|atomics| {
                let mut i = 0;
                $(let $atomic = atomics[i]; i += 1;)*

                let mut row: Vec<CellStruct> = atomics.iter().map(|atomic| atomic.cell().justify(Justify::Center)).collect();
                row.push($proposition($($atomic),*).cell().justify(Justify::Right));
                row
            }).collect::<Vec<Vec<_>>>().table().title(vec![
                $(
                    stringify!($atomic).cell().bold(true),
                )*
                stringify!($proposition).cell().bold(true),
            ]).bold(true);
            // PRINT
            assert!(print_stdout(table).is_ok());
        }
    };
}
