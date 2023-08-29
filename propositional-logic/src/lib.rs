#![deny(clippy::all, clippy::pedantic, clippy::nursery, clippy::perf)]

pub mod macros;
pub mod prelude;

/// The Negation Logical Connective
/// # Example
/// ```
/// use propositional_logic::prelude::*;
///
/// assert_eq!(not(true), false);
/// assert_eq!(not(false), true);
/// ```
#[must_use]
pub const fn not(a: bool) -> bool {
    !a
}

/// The Conjunction Logical Connective
///
/// # Example
/// ```
/// use propositional_logic::prelude::*;
///
/// assert_eq!(and(true, true), true);
/// assert_eq!(and(true, false), false);
/// assert_eq!(and(false, true), false);
/// assert_eq!(and(false, false), false);
/// ```
#[must_use]
pub const fn and(a: bool, b: bool) -> bool {
    a && b
}

/// The Disjunction Logical Connective
///
/// # Example
/// ```
/// use propositional_logic::prelude::*;
///
/// assert_eq!(or(true, true), true);
/// assert_eq!(or(true, false), true);
/// assert_eq!(or(false, true), true);
/// assert_eq!(or(false, false), false);
/// ```
#[must_use]
pub const fn or(a: bool, b: bool) -> bool {
    a || b
}

/// The Exclusive Disjunction Logical Connective
///
/// # Example
/// ```
/// use propositional_logic::prelude::*;
///
/// assert_eq!(xor(true, true), false);
/// assert_eq!(xor(true, false), true);
/// assert_eq!(xor(false, true), true);
/// assert_eq!(xor(false, false), false);
/// ```
#[must_use]
pub const fn xor(a: bool, b: bool) -> bool {
    a ^ b
}

/// The Implication Logical Connective
///
/// # Example
/// ```
/// use propositional_logic::prelude::*;
///
/// assert_eq!(imply(true, true), true);
/// assert_eq!(imply(true, false), false);
/// assert_eq!(imply(false, true), true);
/// assert_eq!(imply(false, false), true);
/// ```
#[must_use]
pub const fn imply(a: bool, b: bool) -> bool {
    !a || b
}

/// The Biconditional Logical Connective
///
/// # Example
/// ```
/// use propositional_logic::prelude::*;
///
/// assert_eq!(iff(true, true), true);
/// assert_eq!(iff(true, false), false);
/// assert_eq!(iff(false, true), false);
/// assert_eq!(iff(false, false), true);
/// ```
#[must_use]
pub const fn iff(a: bool, b: bool) -> bool {
    imply(a, b) && imply(b, a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not() {
        assert!(!not(true));
        assert!(not(false));
    }

    #[test]
    fn test_and() {
        assert!(and(true, true));
        assert!(!and(true, false));
        assert!(!and(false, true));
        assert!(!and(false, false));
    }

    #[test]
    fn test_or() {
        assert!(or(true, true));
        assert!(or(true, false));
        assert!(or(false, true));
        assert!(!or(false, false));
    }

    #[test]
    fn test_xor() {
        assert!(!xor(true, true));
        assert!(xor(true, false));
        assert!(xor(false, true));
        assert!(!xor(false, false));
    }

    #[test]
    fn test_imply() {
        assert!(imply(true, true));
        assert!(!imply(true, false));
        assert!(imply(false, true));
        assert!(imply(false, false));
    }

    #[test]
    fn test_iff() {
        assert!(iff(true, true));
        assert!(!iff(true, false));
        assert!(!iff(false, true));
        assert!(iff(false, false));
    }
}
