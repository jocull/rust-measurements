use length::*;
use super::{assert_almost_eq};

// Metric
#[test]
fn nanometers() {
    let i1 = Length::from_meters(100.0);
    let r1 = i1.as_nanometers();
    
    let i2 = Length::from_nanometers(100.0);
    let r2 = i2.as_meters();
    
    assert_almost_eq(r1, 100000000000.0);
    assert_almost_eq(r2, 1.0E-7);
}

#[test]
fn micrometers() {
    let i1 = Length::from_meters(100.0);
    let r1 = i1.as_micrometers();
    
    let i2 = Length::from_micrometers(100.0);
    let r2 = i2.as_meters();
    
    assert_almost_eq(r1, 100000000.0);
    assert_almost_eq(r2, 0.0001);
}

#[test]
fn millimeters() {
    let i1 = Length::from_meters(100.0);
    let r1 = i1.as_millimeters();
    
    let i2 = Length::from_millimeters(100.0);
    let r2 = i2.as_meters();
    
    assert_almost_eq(r1, 100000.0);
    assert_almost_eq(r2, 0.1);
}

#[test]
fn centimeters() {
    let i1 = Length::from_meters(100.0);
    let r1 = i1.as_centimeters();
    
    let i2 = Length::from_centimeters(100.0);
    let r2 = i2.as_meters();
    
    assert_almost_eq(r1, 10000.0);
    assert_almost_eq(r2, 1.0);
}

#[test]
fn decameter() {
    let i1 = Length::from_meters(100.0);
    let r1 = i1.as_decameters();
    
    let i2 = Length::from_decameters(100.0);
    let r2 = i2.as_meters();
    
    assert_almost_eq(r1, 10.0);
    assert_almost_eq(r2, 1000.0);
}

#[test]
fn hectometer() {
    let i1 = Length::from_meters(100.0);
    let r1 = i1.as_hectometer();
    
    let i2 = Length::from_hectometers(100.0);
    let r2 = i2.as_meters();
    
    assert_almost_eq(r1, 1.0);
    assert_almost_eq(r2, 10000.0);
}

#[test]
fn kilometer() {
    let i1 = Length::from_meters(100.0);
    let r1 = i1.as_kilometers();
    
    let i2 = Length::from_kilometers(100.0);
    let r2 = i2.as_meters();
    
    assert_almost_eq(r1, 0.1);
    assert_almost_eq(r2, 100000.0);
}

// Imperial
#[test]
fn inches() {
    let i1 = Length::from_meters(100.0);
    let r1 = i1.as_inches();
    
    let i2 = Length::from_inches(100.0);
    let r2 = i2.as_meters();
    
    assert_almost_eq(r1, 3937.00787402);
    assert_almost_eq(r2, 2.54);
}

#[test]
fn feet() {
    let i1 = Length::from_meters(100.0);
    let r1 = i1.as_feet();
    
    let i2 = Length::from_feet(100.0);
    let r2 = i2.as_meters();
    
    assert_almost_eq(r1, 328.083989501);
    assert_almost_eq(r2, 30.48);
}

#[test]
fn yards() {
    let i1 = Length::from_meters(100.0);
    let r1 = i1.as_yards();
    
    let i2 = Length::from_yards(100.0);
    let r2 = i2.as_meters();
    
    assert_almost_eq(r1, 109.361329834);
    assert_almost_eq(r2, 91.44);
}

#[test]
fn furlongs() {
    let i1 = Length::from_meters(100.0);
    let r1 = i1.as_furlongs();
    
    let i2 = Length::from_furlongs(100.0);
    let r2 = i2.as_meters();
    
    assert_almost_eq(r1, 0.49709695379);
    assert_almost_eq(r2, 20116.8);
}

#[test]
fn miles() {
    let i1 = Length::from_meters(100.0);
    let r1 = i1.as_miles();
    
    let i2 = Length::from_miles(100.0);
    let r2 = i2.as_meters();
    
    assert_almost_eq(r1, 0.0621371192237);
    assert_almost_eq(r2, 160934.4);
}

// Traits
#[test]
fn add() {
    let a = Length::from_meters(2.0);
    let b = Length::from_meters(4.0);
    let c = a + b;
    assert_almost_eq(c.as_meters(), 6.0);
}

#[test]
fn sub() {
    let a = Length::from_meters(2.0);
    let b = Length::from_meters(4.0);
    let c = a - b;
    assert_almost_eq(c.as_meters(), -2.0);
}

#[test]
fn mul() {
    let a = Length::from_meters(2.0);
    let b = Length::from_meters(4.0);
    let c = a * b;
    let d = b * 2.0;
    assert_almost_eq(c.as_meters(), 8.0);
    assert_almost_eq(d.as_meters(), 8.0);
}

#[test]
fn div() {
    let a = Length::from_meters(2.0);
    let b = Length::from_meters(4.0);
    let c = a * b;
    let d = a * 2.0;
    assert_almost_eq(c.as_meters(), 8.0);
    assert_almost_eq(d.as_meters(), 4.0);
}

#[test]
fn eq() {
    let a = Length::from_meters(2.0);
    let b = Length::from_meters(2.0);
    assert_eq!(a == b, true);
}

#[test]
fn neq() {
    let a = Length::from_meters(2.0);
    let b = Length::from_meters(4.0);
    assert_eq!(a == b, false);
}

#[test]
fn cmp() {
    let a = Length::from_meters(2.0);
    let b = Length::from_meters(4.0);
    assert_eq!(a < b, true);
    assert_eq!(a <= b, true);
    assert_eq!(a > b, false);
    assert_eq!(a >= b, false);
}
