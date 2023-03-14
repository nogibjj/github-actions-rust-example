/* Write unit tests for src/lib.rs */
use trust::add;
use trust::div;
use trust::mul;
use trust::sub;

#[test]
fn test_add() {
    assert_eq!(add(1, 2), 3);
}

#[test]
fn test_sub() {
    assert_eq!(sub(1, 2), -1);
}

#[test]
fn test_mul() {
    assert_eq!(mul(1, 2), 2);
}

#[test]
fn test_div() {
    assert_eq!(div(1, 2), 0);
}
