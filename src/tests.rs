const DEFAULT_DELTA: f64 = 0.000001;

use super::Length;

fn almost_eq(a: f64, b: f64) -> bool {
    almost_eq_delta(a, b, DEFAULT_DELTA)
}

fn almost_eq_delta(a: f64, b: f64, d: f64) -> bool {
    (a - b).abs() < d
}

fn assert_almost_eq(a: f64, b: f64) {
    assert_almost_eq_delta(a, b, DEFAULT_DELTA);
}

fn assert_almost_eq_delta(a: f64, b: f64, d: f64) {
    if !almost_eq_delta(a, b, d) {
        panic!("assertion failed: {:?} != {:?} (within {:?})", a, b, d);
    }
}

#[test]
fn nanometers() {
    // Basic tests
    assert_almost_eq(Length::from_meters(1.0).as_nanometers(), super::METER_NANOMETER_FACTOR);
    assert_almost_eq(Length::from_nanometers(super::METER_NANOMETER_FACTOR).as_meters(), 1.0);
    
    // Floating-point precision tests
    assert_almost_eq(Length::from_nanometers(1.0).as_nanometers(), Length::from_nanometers(1.0).as_nanometers());
    assert_almost_eq(Length::from_nanometers(0.00025).as_nanometers(), Length::from_nanometers(0.00025).as_nanometers());
}

#[test]
fn micrometers() {
    assert_almost_eq(Length::from_meters(1.0).as_micrometers(), super::METER_MICROMETER_FACTOR);
    assert_almost_eq(Length::from_micrometers(super::METER_MICROMETER_FACTOR).as_meters(), 1.0);
}

#[test]
fn millimeters() {
    assert_almost_eq(Length::from_meters(1.0).as_millimeters(), super::METER_MILLIMETER_FACTOR);
    assert_almost_eq(Length::from_millimeters(super::METER_MILLIMETER_FACTOR).as_meters(), 1.0);
}

#[test]
fn centimeters() {
    assert_almost_eq(Length::from_meters(1.0).as_centimeters(), super::METER_CENTIMETER_FACTOR);
    assert_almost_eq(Length::from_centimeters(super::METER_CENTIMETER_FACTOR).as_meters(), 1.0);
}

#[test]
fn decameter() {
    assert_almost_eq(Length::from_meters(1.0).as_decameters(), super::METER_DECAMETER_FACTOR);
    assert_almost_eq(Length::from_decameters(super::METER_DECAMETER_FACTOR).as_meters(), 1.0);
}

#[test]
fn hectometer() {
    assert_almost_eq(Length::from_meters(1.0).as_hectometer(), super::METER_HECTOMETER_FACTOR);
    assert_almost_eq(Length::from_hectometers(super::METER_HECTOMETER_FACTOR).as_meters(), 1.0);
}

#[test]
fn kilometer() {
    assert_almost_eq(Length::from_meters(1.0).as_kilometers(), super::METER_KILOMETER_FACTOR);
    assert_almost_eq(Length::from_kilometers(super::METER_KILOMETER_FACTOR).as_meters(), 1.0);
}

// Imperial
#[test]
fn inches() {
    assert_almost_eq(Length::from_meters(1.0).as_inches(), super::METER_INCH_FACTOR);
    assert_almost_eq(Length::from_inches(super::METER_INCH_FACTOR).as_meters(), 1.0);
}

#[test]
fn feet() {
    assert_almost_eq(Length::from_meters(1.0).as_feet(), super::METER_FEET_FACTOR);
    assert_almost_eq(Length::from_feet(super::METER_FEET_FACTOR).as_meters(), 1.0);
}

#[test]
fn yards() {
    assert_almost_eq(Length::from_meters(1.0).as_yards(), super::METER_YARD_FACTOR);
    assert_almost_eq(Length::from_yards(super::METER_YARD_FACTOR).as_meters(), 1.0);
}

#[test]
fn furlongs() {
    assert_almost_eq(Length::from_meters(201.168).as_furlongs(), 1.0);
    assert_almost_eq(Length::from_furlongs(1.0).as_meters(), 201.168);
}

#[test]
fn miles() {
    assert_almost_eq(Length::from_meters(1609.344).as_miles(), 1.0);
    assert_almost_eq(Length::from_miles(1.0).as_meters(), 1609.344);
}
