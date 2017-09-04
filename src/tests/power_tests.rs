use power::*;
use super::assert_almost_eq;

#[test]
pub fn as_btu_per_minute() {
    let i1 = Power::from_btu_per_minute(100.0);
    let r1 = i1.as_watts();

    let i2 = Power::from_watts(100.0);
    let r2 = i2.as_btu_per_minute();

    assert_almost_eq(r1, 1758.426666666667);
    assert_almost_eq(r2, 5.686901927480627);
}

#[test]
pub fn as_horsepower() {
    let i1 = Power::from_horsepower(100.0);
    let r1 = i1.as_watts();

    let i2 = Power::from_watts(100.0);
    let r2 = i2.as_horsepower();

    assert_almost_eq(r1, 74569.98715822702);
    assert_almost_eq(r2, 0.1341022089595028);
}

#[test]
pub fn as_kilowatts() {
    let i1 = Power::from_kilowatts(100.0);
    let r1 = i1.as_watts();

    let i2 = Power::from_watts(100.0);
    let r2 = i2.as_kilowatts();

    assert_almost_eq(r1, 100000.0);
    assert_almost_eq(r2, 0.1);
}

// Traits
#[test]
fn add() {
    let a = Power::from_watts(2.0);
    let b = Power::from_watts(4.0);
    let c = a + b;
    let d = b + a;
    assert_almost_eq(c.as_watts(), 6.0);
    assert_eq!(c, d);
}

#[test]
fn sub() {
    let a = Power::from_watts(2.0);
    let b = Power::from_watts(4.0);
    let c = a - b;
    assert_almost_eq(c.as_watts(), -2.0);
}

#[test]
fn mul() {
    let a = Power::from_watts(3.0);
    let b = a * 2.0;
    let c = 2.0 * a;
    assert_almost_eq(b.as_watts(), 6.0);
    assert_eq!(b, c);
}

#[test]
fn div() {
    let a = Power::from_watts(2.0);
    let b = Power::from_watts(4.0);
    let c = a / b;
    let d = a / 2.0;
    assert_almost_eq(c, 0.5);
    assert_almost_eq(d.as_watts(), 1.0);
}

#[test]
fn eq() {
    let a = Power::from_watts(2.0);
    let b = Power::from_watts(2.0);
    assert_eq!(a == b, true);
}

#[test]
fn neq() {
    let a = Power::from_watts(2.0);
    let b = Power::from_watts(4.0);
    assert_eq!(a == b, false);
}

#[test]
fn cmp() {
    let a = Power::from_watts(2.0);
    let b = Power::from_watts(4.0);
    assert_eq!(a < b, true);
    assert_eq!(a <= b, true);
    assert_eq!(a > b, false);
    assert_eq!(a >= b, false);
}
