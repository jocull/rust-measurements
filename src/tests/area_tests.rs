use area::*;
use super::assert_almost_eq;

#[test]
fn square_meters() {
	let i1 = Area::from_square_meters(100.0);
	let r1 = i1.as_square_meters();
	let i2 = Area::from_square_meters(100.0);
	let r2 = i2.as_square_meters();
	assert_almost_eq(r1, 100.0);
	assert_almost_eq(r2, 100.0);
}

#[test]
fn square_metres() {
	let i1 = Area::from_square_meters(100.0);
	let r1 = i1.as_square_metres();
	let i2 = Area::from_square_metres(100.0);
	let r2 = i2.as_square_meters();
	assert_almost_eq(r1, 100.0);
	assert_almost_eq(r2, 100.0);
}

#[test]
fn square_nanometers() {
	let i1 = Area::from_square_meters(100.0);
	let r1 = i1.as_square_nanometers();
	let i2 = Area::from_square_nanometers(100.0);
	let r2 = i2.as_square_meters();
	assert_almost_eq(r1, 1e20);
	assert_almost_eq(r2, 1e-16);
}

#[test]
fn square_nanometres() {
	let i1 = Area::from_square_meters(100.0);
	let r1 = i1.as_square_nanometres();
	let i2 = Area::from_square_nanometres(100.0);
	let r2 = i2.as_square_meters();
	assert_almost_eq(r1, 1e20);
	assert_almost_eq(r2, 1e-16);
}

#[test]
fn square_micrometers() {
	let i1 = Area::from_square_meters(100.0);
	let r1 = i1.as_square_micrometers();
	let i2 = Area::from_square_micrometers(100.0);
	let r2 = i2.as_square_meters();
	assert_almost_eq(r1, 1e14);
	assert_almost_eq(r2, 1e-10);
}

#[test]
fn square_micrometres() {
	let i1 = Area::from_square_meters(100.0);
	let r1 = i1.as_square_micrometres();
	let i2 = Area::from_square_micrometres(100.0);
	let r2 = i2.as_square_meters();
	assert_almost_eq(r1, 1e14);
	assert_almost_eq(r2, 1e-10);
}

#[test]
fn square_millimeters() {
	let i1 = Area::from_square_meters(100.0);
	let r1 = i1.as_square_millimeters();
	let i2 = Area::from_square_millimeters(100.0);
	let r2 = i2.as_square_meters();
	assert_almost_eq(r1, 1e8);
	assert_almost_eq(r2, 1e-4);
}

#[test]
fn square_millimetres() {
	let i1 = Area::from_square_meters(100.0);
	let r1 = i1.as_square_millimetres();
	let i2 = Area::from_square_millimetres(100.0);
	let r2 = i2.as_square_meters();
	assert_almost_eq(r1, 1e8);
	assert_almost_eq(r2, 1e-4);
}

#[test]
fn square_centimeters() {
	let i1 = Area::from_square_meters(100.0);
	let r1 = i1.as_square_centimeters();
	let i2 = Area::from_square_centimeters(100.0);
	let r2 = i2.as_square_meters();
	assert_almost_eq(r1, 1e6);
	assert_almost_eq(r2, 1e-2);
}

#[test]
fn square_centimetres() {
	let i1 = Area::from_square_meters(100.0);
	let r1 = i1.as_square_centimetres();
	let i2 = Area::from_square_centimetres(100.0);
	let r2 = i2.as_square_meters();
	assert_almost_eq(r1, 1e6);
	assert_almost_eq(r2, 1e-2);
}

#[test]
fn square_decimeters() {
	let i1 = Area::from_square_meters(100.0);
	let r1 = i1.as_square_decimeters();
	let i2 = Area::from_square_decimeters(100.0);
	let r2 = i2.as_square_meters();
	assert_almost_eq(r1, 1e4);
	assert_almost_eq(r2, 1.0);
}

#[test]
fn square_decimetres() {
	let i1 = Area::from_square_meters(100.0);
	let r1 = i1.as_square_decimetres();
	let i2 = Area::from_square_decimetres(100.0);
	let r2 = i2.as_square_meters();
	assert_almost_eq(r1, 1e4);
	assert_almost_eq(r2, 1.0);
}

#[test]
fn square_hectometers() {
	let i1 = Area::from_square_meters(100.0);
	let r1 = i1.as_square_hectometers();
	let i2 = Area::from_square_hectometers(100.0);
	let r2 = i2.as_square_meters();
	assert_almost_eq(r1, 1e-2);
	assert_almost_eq(r2, 1e6);
}

#[test]
fn square_hectometres() {
	let i1 = Area::from_square_meters(100.0);
	let r1 = i1.as_square_hectometres();
	let i2 = Area::from_square_hectometres(100.0);
	let r2 = i2.as_square_meters();
	assert_almost_eq(r1, 1e-2);
	assert_almost_eq(r2, 1e6);
}

#[test]
fn hectares() {
	let i1 = Area::from_square_meters(100.0);
	let r1 = i1.as_hectares();
	let i2 = Area::from_hectares(100.0);
	let r2 = i2.as_square_meters();
	assert_almost_eq(r1, 1e-2);
	assert_almost_eq(r2, 1e6);
}

#[test]
fn square_kilometers() {
	let i1 = Area::from_square_meters(100.0);
	let r1 = i1.as_square_kilometers();
	let i2 = Area::from_square_kilometers(100.0);
	let r2 = i2.as_square_meters();
	assert_almost_eq(r1, 1e-4);
	assert_almost_eq(r2, 1e8);
}

#[test]
fn square_kilometres() {
	let i1 = Area::from_square_meters(100.0);
	let r1 = i1.as_square_kilometres();
	let i2 = Area::from_square_kilometres(100.0);
	let r2 = i2.as_square_meters();
	assert_almost_eq(r1, 1e-4);
	assert_almost_eq(r2, 1e8);
}

#[test]
fn square_inches() {
	let i1 = Area::from_square_meters(100.0);
	let r1 = i1.as_square_inches();
	let i2 = Area::from_square_inches(100.0);
	let r2 = i2.as_square_meters();
	assert_almost_eq(r1, 155000.0);
	assert_almost_eq(r2, 0.06451587097);
}

#[test]
fn square_feet() {
	let i1 = Area::from_square_meters(100.0);
	let r1 = i1.as_square_feet();
	let i2 = Area::from_square_feet(100.0);
	let r2 = i2.as_square_meters();
	assert_almost_eq(r1, 1076.39);
	assert_almost_eq(r2, 9.2902950097728);
}

#[test]
fn square_yards() {
	let i1 = Area::from_square_meters(100.0);
	let r1 = i1.as_square_yards();
	let i2 = Area::from_square_yards(100.0);
	let r2 = i2.as_square_meters();
	assert_almost_eq(r1, 119.599);
	assert_almost_eq(r2, 83.612732764187);
}

#[test]
fn acres() {
	let i1 = Area::from_square_meters(100.0);
	let r1 = i1.as_acres();
	let i2 = Area::from_acres(100.0);
	let r2 = i2.as_square_meters();
	assert_almost_eq(r1, 0.0247105);
	assert_almost_eq(r2, 404686.0);
}

#[test]
fn square_miles() {
	let i1 = Area::from_square_meters(100.0);
	let r1 = i1.as_square_miles();
	let i2 = Area::from_square_miles(100.0);
	let r2 = i2.as_square_meters();
	assert_almost_eq(r1, 3.86102e-5);
	assert_almost_eq(r2, 258998704.7);
}
