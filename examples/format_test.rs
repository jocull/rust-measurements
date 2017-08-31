extern crate measurements;
use measurements::Temperature;
use measurements::Length;
use measurements::Pressure;
fn main() {
	let t = Temperature::from_celsius(123.456);
	let d = Length::from_meters(123.456);
	let p = Pressure::from_pascals(123.456);
	println!("Temp of {0:.5} outside", t);
	println!("Temp of {0:.5} C outside", t.as_celsius());
	println!("Distance of {0:.5}", d);
	println!("Distance of {0:.5} m", d.as_meters());
	println!("Pressure of {0:.5}", p);
	println!("Pressure of {0:.5} Pa", p.as_pascals());
}
