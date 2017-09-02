extern crate measurements;
use measurements::Temperature;
use measurements::Length;
use measurements::Pressure;
use measurements::Volume;
fn main() {
	let t = Temperature::from_celsius(123.456);
	let d = Length::from_meters(123.456);
	let p = Pressure::from_millibars(123.456);
	let v = Volume::from_litres(123.456);
	println!("Temp of {0:.5} outside", t);
	println!("Distance of {0:.5}", d);
	println!("Pressure of {0:.5}", p);
	println!("Volume of {0:.5}", v);
}
