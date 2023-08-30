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

/// A macro to create a truth table for a given proposition, or propositions. Returns a `::cli_table::TableStruct`
///
/// You can give it both functions and inline propositions, and it will return the truth table of each one.
///
/// You can then print the truth table with `::cli_table::print_stdout(table)`.
///
/// if you only need to PRINT the truth table, you can use the `print_truth_table!` macro for convenience.
///
/// # Example
///
/// ```rust
/// use propositional_logic::prelude::*;
/// use cli_table::print_stdout;
///
/// let compound_proposition = |p, q, r| -> bool { iff(q, (p && !q) || (!p && q)) && r };
///
/// // giving it a function (any function with boolean input and output works, technically)
/// let table = truth_table!(|p, q, r| => compound_proposition);
///
/// // giving it an inline proposition (similar to a closure)
/// let table = truth_table!(|p, q, r| => inline_compound_proposition -> {
///     iff(q, (p && !q) || (!p && q)) && r
/// });
///
/// // giving it both (you can give it as many as you want, but every item must have a comma at the end (even the last one))
/// let table = truth_table!(|p, q, r| => {
///     compound_proposition,
///     inline_compound_proposition -> {
///         iff(q, (p && !q) || (!p && q)) && r
///     }
/// });
///
/// assert!(print_stdout(table).is_ok());
/// ```
///
/// # Output
///
/// ```text
/// +-------+-------+-------+----------------------+-----------------------------+
/// | p     | q     | r     | compound_proposition | inline_compound_proposition |
/// +-------+-------+-------+----------------------+-----------------------------+
/// | true  | true  | true  |                false |                       false |
/// +-------+-------+-------+----------------------+-----------------------------+
/// | true  | true  | false |                false |                       false |
/// +-------+-------+-------+----------------------+-----------------------------+
/// | true  | false | true  |                false |                       false |
/// +-------+-------+-------+----------------------+-----------------------------+
/// | true  | false | false |                false |                       false |
/// +-------+-------+-------+----------------------+-----------------------------+
/// | false | true  | true  |                 true |                        true |
/// +-------+-------+-------+----------------------+-----------------------------+
/// | false | true  | false |                false |                       false |
/// +-------+-------+-------+----------------------+-----------------------------+
/// | false | false | true  |                 true |                        true |
/// +-------+-------+-------+----------------------+-----------------------------+
/// | false | false | false |                false |                       false |
/// +-------+-------+-------+----------------------+-----------------------------+
/// ```
#[macro_export]
macro_rules! truth_table {
    // catch-all input parser
    (|$($atomic:ident),*| => { $($rest:tt)* } ) => {{
        // send the glob to the TT Munchers to get a bunch of expressions
        truth_table!(@muncher |$($atomic),*|; {$($rest)*} )
    }};
    // only giving one inline proposition
    (|$($atomic:ident),*| => $proposition:ident -> $body:expr) => {{
        let $proposition = | $($atomic: bool),* | -> bool {
            $body
        };
        truth_table!(@muncher |$($atomic),*| $proposition; {} )
    }};
    // only giving one expression
    (|$($atomic:ident),*| => $proposition:ident) => {{
        truth_table!(@muncher |$($atomic),*| $proposition; {} )
    }};
    // parser for inline syntax
    (@muncher |$($atomic:ident),*| $($previous:ident),*; { $next:ident -> $body:expr, $($rest:tt)* }) => {{
        // send the glob to the TT Munchers to get a bunch of expressions
        let $next = | $($atomic: bool),* | -> bool {
            $body
        };
        truth_table!(
            @muncher
            |$($atomic),*|
            $($previous,)* $next;
            {
                $($rest)*
            }
        )
    }};
    // parser for function syntax
    (@muncher |$($atomic:ident),*| $($previous:ident),*; { $next:ident, $($rest:tt)* }) => {{
        truth_table!(
            @muncher
            |$($atomic),*|
            $($previous,)* $next;
            {$($rest)*}
        )
    }};
    // parser for inline syntax (last item)
    (@muncher |$($atomic:ident),*| $($previous:ident),*; { $next:ident -> $body:expr }) => {{
        // send the glob to the TT Munchers to get a bunch of expressions
        let $next = | $($atomic: bool),* | -> bool {
            $body
        };
        truth_table!(
            @muncher
            |$($atomic),*|
            $($previous,)* $next;
            {}
        )
    }};
    // parser for function syntax (last item)
    (@muncher |$($atomic:ident),*| $($previous:ident),*; { $next:ident }) => {{
        truth_table!(
            @muncher
            |$($atomic),*|
            $($previous,)* $next;
            {}
        )
    }};
    // the final state of the parser
    (@muncher |$($atomic:ident),*| $($compound_proposition:ident),*; {}) => {{
        use ::std::convert::identity;

        use ::cli_table::{format::Justify, Cell, CellStruct, Style, Table};

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
            let _ = i; // a no-op to make sure the compiler knows that i is used

            let mut row: Vec<CellStruct> = atomics.iter().map(|atomic| atomic.cell().justify(Justify::Center)).collect();
            let run_proposition = |proposition: fn($($atomic: bool),*) -> bool |-> bool {proposition($($atomic),*)};
            $(
                row.push(run_proposition($compound_proposition).cell().justify(Justify::Right));
            )*

            row
        }).collect::<Vec<Vec<_>>>().table().title(vec![
            $(
                stringify!($atomic).cell().bold(true),
            )*
            $(
                stringify!($compound_proposition).cell().bold(true),
            )*
        ]).bold(true);
        table
    }};
}

/// A macro to print a truth table for the given propositions.
///
/// # Note
///
/// you can't easily have inline propositions depend on other propositions, but any function with boolean input/output works so you can use that to get around it.
///
/// For Example:
/// ```rust
/// # use propositional_logic::prelude::*;
///
/// fn foo(p: bool, _: bool) -> bool { not(p) }
/// fn bar(p: bool, q: bool) -> bool { p & foo(q, p) }
///
/// print_truth_table!(|p, q| => {
///    foo,
///    bar,
/// });
/// ```
///
/// # Examples
///
/// ```rust
/// use propositional_logic::prelude::*;
///
/// let compound_proposition = |p, q| -> bool { iff(q, p) };
///
/// // giving it a function (any function with boolean input and output works, technically)
/// print_truth_table!(|p, q| => compound_proposition);
/// println!();
/// // giving it an inline proposition (similar to a closure)
/// print_truth_table!(|p| => inline_compound_proposition -> {
///     not(p)
/// });
/// println!();
/// // giving it both (you can give it as many as you want)
/// print_truth_table!(|p,q| => {
///     compound_proposition,
///     inline_compound_proposition -> {
///         not(p)
///     }
/// });
/// ```
///
/// # Output
///
/// ```text
/// +-------+-------+----------------------+
/// | p     | q     | compound_proposition |
/// +-------+-------+----------------------+
/// | true  | true  |                 true |
/// +-------+-------+----------------------+
/// | true  | false |                false |
/// +-------+-------+----------------------+
/// | false | true  |                false |
/// +-------+-------+----------------------+
/// | false | false |                 true |
/// +-------+-------+----------------------+
///
/// +-------+-----------------------------+
/// | p     | inline_compound_proposition |
/// +-------+-----------------------------+
/// | true  |                       false |
/// +-------+-----------------------------+
/// | false |                        true |
/// +-------+-----------------------------+
///
/// +-------+-------+----------------------+-----------------------------+
/// | p     | q     | compound_proposition | inline_compound_proposition |
/// +-------+-------+----------------------+-----------------------------+
/// | true  | true  |                 true |                       false |
/// +-------+-------+----------------------+-----------------------------+
/// | true  | false |                false |                       false |
/// +-------+-------+----------------------+-----------------------------+
/// | false | true  |                false |                        true |
/// +-------+-------+----------------------+-----------------------------+
/// | false | false |                 true |                        true |
/// +-------+-------+----------------------+-----------------------------+
/// ```
#[macro_export]
macro_rules! print_truth_table {
    // catch-all input parser
    (|$($atomic:ident),*| => {
        $($rest:tt)*
    }) => {{
        // send the glob to the TT Munchers to get a bunch of expressions
        let table = truth_table!(|$($atomic),*| => { $($rest)* } );
        assert!(::cli_table::print_stdout(table).is_ok());
    }};
    // only giving one inline proposition
    (|$($atomic:ident),*| => $prop:ident -> $body:expr) => {{
        // send the glob to the TT Munchers to get a bunch of expressions
        let table = truth_table!(|$($atomic),*| => $prop -> {$body}  );
        assert!(::cli_table::print_stdout(table).is_ok());
    }};
    // only giving one expression
    (|$($atomic:ident),*| => $prop:ident) => {{
        let table = truth_table!(|$($atomic),*| => $prop );
        assert!(::cli_table::print_stdout(table).is_ok());
    }};

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
