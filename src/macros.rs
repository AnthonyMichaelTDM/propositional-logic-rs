/// Returns a vector of all possible truth values for a given number of atomic propositions.
#[must_use]
pub fn possible_truth_values(n: usize) -> Vec<Vec<bool>> {
    if n == 0 {
        return vec![vec![]];
    }

    let mut truth_values = Vec::new();

    for i in (0..(2 << (n - 1))).rev() {
        let mut truth_value = Vec::new();

        for j in (0..n).rev() {
            truth_value.push((i >> j) % 2 == 1);
        }

        truth_values.push(truth_value);
    }

    truth_values
}

pub use cli_table;

/// A macro to print a truth table for a given proposition.
///
/// # Example
///
/// ```rust
/// use propositional_logic::prelude::*;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_possible_truth_values_dimensions() {
        // ensure the dimensions are correct up to 20 atomic propositions (2^20 = 1048576 rows)
        for i in 0..20 {
            let truth_values = possible_truth_values(i);
            assert_eq!(truth_values.len(), 2usize.pow(i as u32));
            for truth_value in truth_values {
                assert_eq!(truth_value.len(), i);
            }
        }
    }

    #[test]
    fn test_possible_truth_values() {
        let expected = vec![
            vec![true, true, true, true],
            vec![true, true, true, false],
            vec![true, true, false, true],
            vec![true, true, false, false],
            vec![true, false, true, true],
            vec![true, false, true, false],
            vec![true, false, false, true],
            vec![true, false, false, false],
            vec![false, true, true, true],
            vec![false, true, true, false],
            vec![false, true, false, true],
            vec![false, true, false, false],
            vec![false, false, true, true],
            vec![false, false, true, false],
            vec![false, false, false, true],
            vec![false, false, false, false],
        ];
        assert_eq!(possible_truth_values(4), expected);
        let expected = vec![
            vec![true, true, true],
            vec![true, true, false],
            vec![true, false, true],
            vec![true, false, false],
            vec![false, true, true],
            vec![false, true, false],
            vec![false, false, true],
            vec![false, false, false],
        ];
        assert_eq!(possible_truth_values(3), expected);
        let expected = vec![
            vec![true, true],
            vec![true, false],
            vec![false, true],
            vec![false, false],
        ];
        assert_eq!(possible_truth_values(2), expected);
        let expected = vec![vec![true], vec![false]];
        assert_eq!(possible_truth_values(1), expected);
        let expected = vec![vec![]];
        assert_eq!(possible_truth_values(0), expected);
    }
}
