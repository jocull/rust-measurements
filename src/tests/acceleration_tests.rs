use acceleration::*;
use super::assert_almost_eq;
use std::time::Duration;
use speed::Speed;

// Metric
#[test]
fn speed_over_time() {
    let s1 = Speed::from_meters_per_second(10.0);
    let t1 = Duration::new(5, 0);
    let i1 = s1 / t1;
    let r1 = i1.as_meters_per_second_per_second();
    assert_almost_eq(r1, 2.0);
}

// Traits
#[test]
fn add() {
    let a = Acceleration::from_meters_per_second_per_second(2.0);
    let b = Acceleration::from_meters_per_second_per_second(4.0);
    let c = a + b;
    let d = b + a;
    assert_almost_eq(c.as_meters_per_second_per_second(), 6.0);
    assert_eq!(c, d);
}

#[test]
fn sub() {
    let a = Acceleration::from_meters_per_second_per_second(2.0);
    let b = Acceleration::from_meters_per_second_per_second(4.0);
    let c = a - b;
    assert_almost_eq(c.as_meters_per_second_per_second(), -2.0);
}

#[test]
fn mul() {
    let a = Acceleration::from_meters_per_second_per_second(3.0);
    let b = a * 2.0;
    let c = 2.0 * a;
    assert_almost_eq(b.as_meters_per_second_per_second(), 6.0);
    assert_eq!(b, c);
}

#[test]
fn div() {
    let a = Acceleration::from_meters_per_second_per_second(2.0);
    let b = Acceleration::from_meters_per_second_per_second(4.0);
    let c = a / b;
    let d = a / 2.0;
    assert_almost_eq(c, 0.5);
    assert_almost_eq(d.as_meters_per_second_per_second(), 1.0);
}

#[test]
fn eq() {
    let a = Acceleration::from_meters_per_second_per_second(2.0);
    let b = Acceleration::from_meters_per_second_per_second(2.0);
    assert_eq!(a == b, true);
}

#[test]
fn neq() {
    let a = Acceleration::from_meters_per_second_per_second(2.0);
    let b = Acceleration::from_meters_per_second_per_second(4.0);
    assert_eq!(a == b, false);
}

#[test]
fn cmp() {
    let a = Acceleration::from_meters_per_second_per_second(2.0);
    let b = Acceleration::from_meters_per_second_per_second(4.0);
    assert_eq!(a < b, true);
    assert_eq!(a <= b, true);
    assert_eq!(a > b, false);
    assert_eq!(a >= b, false);
}
