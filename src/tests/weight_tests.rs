use weight::*;
use super::assert_almost_eq;

// Weight Units
// Metric
#[test]
fn kilograms() {
    let t = Weight::from_kilograms(100.0);
    let o = t.as_kilograms();
    assert_almost_eq(o, 100.0);
}

#[test]
fn micrograms() {
    let t = Weight::from_kilograms(100.0);
    let o = t.as_micrograms();
    assert_almost_eq(o, 1e+11);

    let t = Weight::from_micrograms(100.0);
    let o = t.as_kilograms();
    assert_almost_eq(o, 1e-7);
}

#[test]
fn milligrams() {
    let t = Weight::from_kilograms(100.0);
    let o = t.as_milligrams();
    assert_almost_eq(o, 1e+8);

    let t = Weight::from_milligrams(100.0);
    let o = t.as_kilograms();
    assert_almost_eq(o, 0.0001);
}

#[test]
fn carats() {
    let t = Weight::from_kilograms(100.0);
    let o = t.as_carats();
    assert_almost_eq(o, 500000.0);

    let t = Weight::from_carats(100.0);
    let o = t.as_kilograms();
    assert_almost_eq(o, 0.02);
}

#[test]
fn grams() {
    let t = Weight::from_kilograms(100.0);
    let o = t.as_grams();
    assert_almost_eq(o, 100000.0);

    let t = Weight::from_grams(100.0);
    let o = t.as_kilograms();
    assert_almost_eq(o, 0.1);
}

#[test]
fn metric_tons() {
    let t = Weight::from_kilograms(100.0);
    let o = t.as_metric_tons();
    assert_almost_eq(o, 0.1);

    let t = Weight::from_metric_tons(100.0);
    let o = t.as_kilograms();
    assert_almost_eq(o, 100000.0);
}

// Imperial
#[test]
fn grains() {
    let t = Weight::from_kilograms(1.0);
    let o = t.as_grains();
    assert_almost_eq(o, 15432.358);

    let t = Weight::from_grains(100.0);
    let o = t.as_kilograms();
    assert_almost_eq(o, 0.0064798911);
}

#[test]
fn pennyweights() {
    let t = Weight::from_kilograms(1.0);
    let o = t.as_pennyweights();
    assert_almost_eq(o, 643.01493);

    let t = Weight::from_pennyweights(100.0);
    let o = t.as_kilograms();
    assert_almost_eq(o, 0.15551738);
}

#[test]
fn ounces() {
    let t = Weight::from_kilograms(1.0);
    let o = t.as_ounces();
    assert_almost_eq(o, 35.273962);

    let t = Weight::from_ounces(100.0);
    let o = t.as_kilograms();
    assert_almost_eq(o, 2.8349523);
}

#[test]
fn troy_ounces() {
    let t = Weight::from_kilograms(1.0);
    let o = t.as_troy_ounces();
    assert_almost_eq(o, 32.150747);

    let t = Weight::from_troy_ounces(100.0);
    let o = t.as_kilograms();
    assert_almost_eq(o, 3.1103476);
}

#[test]
fn pounds() {
    let t = Weight::from_kilograms(1.0);
    let o = t.as_pounds();
    assert_almost_eq(o, 2.2046228);

    let t = Weight::from_pounds(100.0);
    let o = t.as_kilograms();
    assert_almost_eq(o, 45.359233);
}

#[test]
fn troy_pounds() {
    let t = Weight::from_kilograms(1.0);
    let o = t.as_troy_pounds();
    assert_almost_eq(o, 2.6792289);

    let t = Weight::from_troy_pounds(100.0);
    let o = t.as_kilograms();
    assert_almost_eq(o, 37.324172);
}

#[test]
fn stones() {
    let t = Weight::from_kilograms(100.0);
    let o = t.as_stones();
    assert_almost_eq(o, 15.74730);

    let t = Weight::from_stones(100.0);
    let o = t.as_kilograms();
    assert_almost_eq(o, 635.02934);
}

#[test]
fn short_tons() {
    let t = Weight::from_kilograms(100.0);
    let o = t.as_short_tons();
    assert_almost_eq(o, 0.11023113);

    let t = Weight::from_short_tons(1.0);
    let o = t.as_kilograms();
    assert_almost_eq(o, 907.18475);
}

#[test]
fn long_tons() {
    let t = Weight::from_kilograms(100.0);
    let o = t.as_long_tons();
    assert_almost_eq(o, 0.098420653);

    let t = Weight::from_long_tons(1.0);
    let o = t.as_kilograms();
    assert_almost_eq(o, 1016.0469);
}

// Traits
#[test]
fn add() {
    let a = Weight::from_kilograms(2.0);
    let b = Weight::from_kilograms(4.0);
    let c = a + b;
    let d = b + a;
    assert_almost_eq(c.as_kilograms(), 6.0);
    assert_eq!(c, d);
}

#[test]
fn sub() {
    let a = Weight::from_kilograms(2.0);
    let b = Weight::from_kilograms(4.0);
    let c = a - b;
    assert_almost_eq(c.as_kilograms(), -2.0);
}

#[test]
fn mul() {
    let a = Weight::from_kilograms(3.0);
    let b = a * 2.0;
    let c = 2.0 * a;
    assert_almost_eq(b.as_kilograms(), 6.0);
    assert_eq!(b, c);
}

#[test]
fn div() {
    let a = Weight::from_kilograms(2.0);
    let b = Weight::from_kilograms(4.0);
    let c = a / b;
    let d = a / 2.0;
    assert_almost_eq(c, 0.5);
    assert_almost_eq(d.as_kilograms(), 1.0);
}

#[test]
fn eq() {
    let a = Weight::from_kilograms(2.0);
    let b = Weight::from_kilograms(2.0);
    assert_eq!(a == b, true);
}

#[test]
fn neq() {
    let a = Weight::from_kilograms(2.0);
    let b = Weight::from_kilograms(4.0);
    assert_eq!(a == b, false);
}

#[test]
fn cmp() {
    let a = Weight::from_kilograms(2.0);
    let b = Weight::from_kilograms(4.0);
    assert_eq!(a < b, true);
    assert_eq!(a <= b, true);
    assert_eq!(a > b, false);
    assert_eq!(a >= b, false);
}
