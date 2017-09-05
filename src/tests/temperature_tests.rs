use temperature::*;
use super::assert_almost_eq;

// Temperature Units
#[test]
fn kelvin() {
    let t = Temperature::from_kelvin(100.0);
    let o = t.as_kelvin();

    assert_almost_eq(o, 100.0);
}

#[test]
fn celsius() {
    let t = Temperature::from_kelvin(100.0);
    let o = t.as_celsius();

    assert_almost_eq(o, -173.15);
}

#[test]
fn fahrenheit() {
    let t = Temperature::from_kelvin(100.0);
    let o = t.as_fahrenheit();

    assert_almost_eq(o, -279.67);
}

#[test]
fn rankine() {
    let t = Temperature::from_kelvin(100.0);
    let o = t.as_rankine();

    assert_almost_eq(o, 180.0);
}

// Traits
#[test]
fn add() {
    let a = Temperature::from_kelvin(2.0);
    let b = TemperatureDelta::from_kelvin(4.0);
    let c = a + b;
    assert_almost_eq(c.as_kelvin(), 6.0);
}

#[test]
fn sub() {
    let a = Temperature::from_kelvin(4.0);
    let b = TemperatureDelta::from_kelvin(2.0);
    let c = a - b;
    assert_almost_eq(c.as_kelvin(), 2.0);
}

#[test]
fn sub2() {
    let a = Temperature::from_fahrenheit(212.0);
    let b = Temperature::from_celsius(75.0);
    let c = a - b;
    assert_almost_eq(c.as_kelvin(), 25.0);
}

#[test]
fn eq() {
    let a = Temperature::from_kelvin(2.0);
    let b = Temperature::from_kelvin(2.0);
    assert_eq!(a == b, true);
}

#[test]
fn neq() {
    let a = Temperature::from_kelvin(2.0);
    let b = Temperature::from_kelvin(4.0);
    assert_eq!(a == b, false);
}

#[test]
fn cmp() {
    let a = Temperature::from_kelvin(2.0);
    let b = Temperature::from_kelvin(4.0);
    assert_eq!(a < b, true);
    assert_eq!(a <= b, true);
    assert_eq!(a > b, false);
    assert_eq!(a >= b, false);
}
