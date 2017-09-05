use volume::*;
use super::assert_almost_eq;

// Volume Units
// Metric
#[test]
fn litres() {
    let t = Volume::from_litres(100.0);
    let o = t.as_litres();
    assert_almost_eq(o, 100.0);
}

#[test]
fn cubic_centimeters() {
    let t = Volume::from_litres(1.0);
    let o = t.as_cubic_centimeters();
    assert_almost_eq(o, 1000.0);

    let t = Volume::from_cubic_centimeters(1000.0);
    let o = t.as_litres();
    assert_almost_eq(o, 1.0);
}

#[test]
fn milliliters() {
    let t = Volume::from_litres(1.0);
    let o = t.as_milliliters();
    assert_almost_eq(o, 1000.0);

    let t = Volume::from_milliliters(1000.0);
    let o = t.as_litres();
    assert_almost_eq(o, 1.0);
}

#[test]
fn cubic_meters() {
    let t = Volume::from_litres(100.0);
    let o = t.as_cubic_meters();
    assert_almost_eq(o, 0.1);

    let t = Volume::from_cubic_meters(100.0);
    let o = t.as_litres();
    assert_almost_eq(o, 100000.0);
}

// Imperial
#[test]
fn drops() {
    let t = Volume::from_litres(100.0);
    let o = t.as_drops();
    assert_almost_eq(o, 1541962.98055);

    let t = Volume::from_drops(100.0);
    let o = t.as_litres();
    assert_almost_eq(o, 0.00648524);
}

#[test]
fn drams() {
    let t = Volume::from_litres(100.0);
    let o = t.as_drams();
    assert_almost_eq(o, 27051.0351863);

    let t = Volume::from_drams(100.0);
    let o = t.as_litres();
    assert_almost_eq(o, 0.36967162);
}

#[test]
fn teaspoons() {
    let t = Volume::from_litres(100.0);
    let o = t.as_teaspoons();
    assert_almost_eq(o, 20288.41362);

    let t = Volume::from_teaspoons(100.0);
    let o = t.as_litres();
    assert_almost_eq(o, 0.492892159402);
}

#[test]
fn tablespoons() {
    let t = Volume::from_litres(100.0);
    let o = t.as_tablespoons();
    assert_almost_eq(o, 6762.80454);

    let t = Volume::from_tablespoons(100.0);
    let o = t.as_litres();
    assert_almost_eq(o, 1.47867647821);
}

#[test]
fn cubic_inches() {
    let t = Volume::from_litres(100.0);
    let o = t.as_cubic_inches();
    assert_almost_eq(o, 6102.37440947);

    let t = Volume::from_cubic_inches(100.0);
    let o = t.as_litres();
    assert_almost_eq(o, 1.6387064);
}

#[test]
fn fluid_ounces_uk() {
    let t = Volume::from_litres(100.0);
    let o = t.as_fluid_ounces_uk();
    assert_almost_eq(o, 3519.506424);

    let t = Volume::from_fluid_ounces_uk(100.0);
    let o = t.as_litres();
    assert_almost_eq(o, 2.84130750034);
}

#[test]
fn fluid_ounces() {
    let t = Volume::from_litres(100.0);
    let o = t.as_fluid_ounces();
    assert_almost_eq(o, 3381.40227);

    let t = Volume::from_fluid_ounces(100.0);
    let o = t.as_litres();
    assert_almost_eq(o, 2.95735295641);
}

#[test]
fn cups() {
    let t = Volume::from_litres(100.0);
    let o = t.as_cups();
    assert_almost_eq(o, 422.6752838);

    let t = Volume::from_cups(100.0);
    let o = t.as_litres();
    assert_almost_eq(o, 23.6588236485);
}

#[test]
fn pints() {
    let t = Volume::from_litres(100.0);
    let o = t.as_pints();
    assert_almost_eq(o, 211.337641887);

    let t = Volume::from_pints(100.0);
    let o = t.as_litres();
    assert_almost_eq(o, 47.3176473);
}

#[test]
fn pints_uk() {
    let t = Volume::from_litres(100.0);
    let o = t.as_pints_uk();
    assert_almost_eq(o, 175.975398639);

    let t = Volume::from_pints_uk(100.0);
    let o = t.as_litres();
    assert_almost_eq(o, 56.826125);
}

#[test]
fn quarts() {
    let t = Volume::from_litres(100.0);
    let o = t.as_quarts();
    assert_almost_eq(o, 105.668820943);

    let t = Volume::from_quarts(100.0);
    let o = t.as_litres();
    assert_almost_eq(o, 94.6352946);
}

#[test]
fn gallons() {
    let t = Volume::from_litres(100.0);
    let o = t.as_gallons();
    assert_almost_eq(o, 26.4172052358);

    let t = Volume::from_gallons(100.0);
    let o = t.as_litres();
    assert_almost_eq(o, 378.5411784);
}

#[test]
fn gallons_uk() {
    let t = Volume::from_litres(100.0);
    let o = t.as_gallons_uk();
    assert_almost_eq(o, 21.9969248299);

    let t = Volume::from_gallons_uk(100.0);
    let o = t.as_litres();
    assert_almost_eq(o, 454.609);
}

#[test]
fn cubic_feet() {
    let t = Volume::from_litres(100.0);
    let o = t.as_cubic_feet();
    assert_almost_eq(o, 3.53146667215);

    let t = Volume::from_cubic_feet(100.0);
    let o = t.as_litres();
    assert_almost_eq(o, 2831.6846592);
}

#[test]
fn cubic_yards() {
    let t = Volume::from_litres(100.0);
    let o = t.as_cubic_yards();
    assert_almost_eq(o, 0.13079506193);

    let t = Volume::from_cubic_yards(100.0);
    let o = t.as_litres();
    assert_almost_eq(o, 76455.4857992);
}

// Traits
#[test]
fn add() {
    let a = Volume::from_litres(2.0);
    let b = Volume::from_litres(4.0);
    let c = a + b;
    let d = b + a;
    assert_almost_eq(c.as_litres(), 6.0);
    assert_eq!(c, d);
}

#[test]
fn sub() {
    let a = Volume::from_litres(2.0);
    let b = Volume::from_litres(4.0);
    let c = a - b;
    assert_almost_eq(c.as_litres(), -2.0);
}

#[test]
fn mul() {
    let a = Volume::from_litres(3.0);
    let b = a * 2.0;
    let c = 2.0 * a;
    assert_almost_eq(b.as_litres(), 6.0);
    assert_eq!(b, c);
}

#[test]
fn div() {
    let a = Volume::from_litres(2.0);
    let b = Volume::from_litres(4.0);
    let c = a / b;
    let d = a / 2.0;
    assert_almost_eq(c, 0.5);
    assert_almost_eq(d.as_litres(), 1.0);
}

#[test]
fn eq() {
    let a = Volume::from_litres(2.0);
    let b = Volume::from_litres(2.0);
    assert_eq!(a == b, true);
}

#[test]
fn neq() {
    let a = Volume::from_litres(2.0);
    let b = Volume::from_litres(4.0);
    assert_eq!(a == b, false);
}

#[test]
fn cmp() {
    let a = Volume::from_litres(2.0);
    let b = Volume::from_litres(4.0);
    assert_eq!(a < b, true);
    assert_eq!(a <= b, true);
    assert_eq!(a > b, false);
    assert_eq!(a >= b, false);
}
