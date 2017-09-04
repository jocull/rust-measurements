use speed::*;
use super::assert_almost_eq;

// Metric
#[test]
fn kilometers_per_hour() {
    let i1 = Speed::from_meters_per_second(100.0);
    let r1 = i1.as_kilometers_per_hour();

    let i2 = Speed::from_kilometers_per_hour(100.0);
    let r2 = i2.as_meters_per_second();

    assert_almost_eq(r1, 360.0);
    assert_almost_eq(r2, 27.7777777777);
}

fn kilometres_per_hour() {
    let i1 = Speed::from_metres_per_second(100.0);
    let r1 = i1.as_kilometres_per_hour();

    let i2 = Speed::from_kilometres_per_hour(100.0);
    let r2 = i2.as_metres_per_second();

    assert_almost_eq(r1, 360.0);
    assert_almost_eq(r2, 27.7777777777);
}

// Imperial
#[test]
fn miles_per_hour() {
    let i1 = Speed::from_meters_per_second(100.0);
    let r1 = i1.as_miles_per_hour();

    let i2 = Speed::from_miles_per_hour(100.0);
    let r2 = i2.as_meters_per_second();

    assert_almost_eq(r1, 223.7414543194530764449968924798);
    assert_almost_eq(r2, 44.694444444444444444444444444444);
}

// Traits
#[test]
fn add() {
    let a = Speed::from_meters_per_second(2.0);
    let b = Speed::from_meters_per_second(4.0);
    let c = a + b;
    let d = b + a;
    assert_almost_eq(c.as_meters_per_second(), 6.0);
    assert_eq!(c, d);
}

#[test]
fn sub() {
    let a = Speed::from_meters_per_second(2.0);
    let b = Speed::from_meters_per_second(4.0);
    let c = a - b;
    assert_almost_eq(c.as_meters_per_second(), -2.0);
}

#[test]
fn mul() {
    let a = Speed::from_meters_per_second(3.0);
    let b = a * 2.0;
    let c = 2.0 * a;
    assert_almost_eq(b.as_meters_per_second(), 6.0);
    assert_eq!(b, c);
}

#[test]
fn div() {
    let a = Speed::from_meters_per_second(2.0);
    let b = Speed::from_meters_per_second(4.0);
    let c = a / b;
    let d = a / 2.0;
    assert_almost_eq(c, 0.5);
    assert_almost_eq(d.as_meters_per_second(), 1.0);
}

#[test]
fn eq() {
    let a = Speed::from_meters_per_second(2.0);
    let b = Speed::from_meters_per_second(2.0);
    assert_eq!(a == b, true);
}

#[test]
fn neq() {
    let a = Speed::from_meters_per_second(2.0);
    let b = Speed::from_meters_per_second(4.0);
    assert_eq!(a == b, false);
}

#[test]
fn cmp() {
    let a = Speed::from_meters_per_second(2.0);
    let b = Speed::from_meters_per_second(4.0);
    assert_eq!(a < b, true);
    assert_eq!(a <= b, true);
    assert_eq!(a > b, false);
    assert_eq!(a >= b, false);
}
