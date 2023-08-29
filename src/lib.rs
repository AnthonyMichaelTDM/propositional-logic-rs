pub mod macros;
pub mod prelude;

// negation is achieved by using the ! operator
pub fn not(a: bool) -> bool {
    !a
}

#[test]
fn test_not() {
    assert_eq!(not(true), false);
    assert_eq!(not(false), true);
}

// Conjunction is achieved by using the && operator
pub fn and(a: bool, b: bool) -> bool {
    a && b
}

#[test]
fn test_and() {
    assert_eq!(and(true, true), true);
    assert_eq!(and(true, false), false);
    assert_eq!(and(false, true), false);
    assert_eq!(and(false, false), false);
}

// Disjunction is achieved by using the || operator
pub fn or(a: bool, b: bool) -> bool {
    a || b
}

#[test]
fn test_or() {
    assert_eq!(or(true, true), true);
    assert_eq!(or(true, false), true);
    assert_eq!(or(false, true), true);
    assert_eq!(or(false, false), false);
}

// Exclusive disjunction is achieved by using the ^ operator
pub fn xor(a: bool, b: bool) -> bool {
    a ^ b
}

#[test]
fn test_xor() {
    assert_eq!(xor(true, true), false);
    assert_eq!(xor(true, false), true);
    assert_eq!(xor(false, true), true);
    assert_eq!(xor(false, false), false);
}

// a -> b
pub fn imply(a: bool, b: bool) -> bool {
    !a || b
}

#[test]
fn test_imply() {
    assert_eq!(imply(true, true), true);
    assert_eq!(imply(true, false), false);
    assert_eq!(imply(false, true), true);
    assert_eq!(imply(false, false), true);
}

// a <-> b
pub fn iff(a: bool, b: bool) -> bool {
    imply(a, b) && imply(b, a)
}

#[test]
fn test_iff() {
    assert_eq!(iff(true, true), true);
    assert_eq!(iff(true, false), false);
    assert_eq!(iff(false, true), false);
    assert_eq!(iff(false, false), true);
}
