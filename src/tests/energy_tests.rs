use energy::*;
use super::assert_almost_eq;


#[test]
pub fn as_kcalories() {
    let i1 = Energy::from_kcalories(100.0);
    let r1 = i1.as_joules();

    let i2 = Energy::from_joules(100.0);
    let r2 = i2.as_kcalories();

    assert_almost_eq(r1, 418680.0);
    assert_almost_eq(r2, 0.0238845896627496);
}

#[test]
pub fn as_btu() {
    let i1 = Energy::from_btu(100.0);
    let r1 = i1.as_joules();

    let i2 = Energy::from_joules(100.0);
    let r2 = i2.as_btu();

    assert_almost_eq(r1, 105505.6);
    assert_almost_eq(r2, 0.0947816987913438);
}

#[test]
pub fn as_e_v() {
    let i1 = Energy::from_e_v(100.0);
    let r1 = i1.as_joules();

    let i2 = Energy::from_joules(100.0);
    let r2 = i2.as_e_v();

    assert_almost_eq(r1, 1.60217653e-17);
    assert_almost_eq(r2, 6.241509479607718e+20);
}

#[test]
pub fn as_watt_hours() {
    let i1 = Energy::from_watt_hours(100.0);
    let r1 = i1.as_joules();

    let i2 = Energy::from_joules(100.0);
    let r2 = i2.as_watt_hours();

    assert_almost_eq(r1, 360000.0);
    assert_almost_eq(r2, 0.02777777777777777777777777777778);
}

#[test]
pub fn as_kilowatt_hours() {
    let i1 = Energy::from_kilowatt_hours(100.0);
    let r1 = i1.as_joules();

    let i2 = Energy::from_joules(100.0);
    let r2 = i2.as_kilowatt_hours();

    assert_almost_eq(r1, 360000000.0);
    assert_almost_eq(r2, 2.777777777777777777777777777778e-5);
}

// Traits
#[test]
fn add() {
    let a = Energy::from_joules(2.0);
    let b = Energy::from_joules(4.0);
    let c = a + b;
    let d = b + a;
    assert_almost_eq(c.as_joules(), 6.0);
    assert_eq!(c, d);
}

#[test]
fn sub() {
    let a = Energy::from_joules(2.0);
    let b = Energy::from_joules(4.0);
    let c = a - b;
    assert_almost_eq(c.as_joules(), -2.0);
}

#[test]
fn mul() {
    let a = Energy::from_joules(3.0);
    let b = a * 2.0;
    let c = 2.0 * a;
    assert_almost_eq(b.as_joules(), 6.0);
    assert_eq!(b, c);
}

#[test]
fn div() {
    let a = Energy::from_joules(2.0);
    let b = Energy::from_joules(4.0);
    let c = a / b;
    let d = a / 2.0;
    assert_almost_eq(c, 0.5);
    assert_almost_eq(d.as_joules(), 1.0);
}

#[test]
fn eq() {
    let a = Energy::from_joules(2.0);
    let b = Energy::from_joules(2.0);
    assert_eq!(a == b, true);
}

#[test]
fn neq() {
    let a = Energy::from_joules(2.0);
    let b = Energy::from_joules(4.0);
    assert_eq!(a == b, false);
}

#[test]
fn cmp() {
    let a = Energy::from_joules(2.0);
    let b = Energy::from_joules(4.0);
    assert_eq!(a < b, true);
    assert_eq!(a <= b, true);
    assert_eq!(a > b, false);
    assert_eq!(a >= b, false);
}
