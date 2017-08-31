extern crate measurements;
use measurements::Temperature;
use measurements::Length;
fn main() {
	let t = Temperature::from_celsius(0.1);
	let d = Length::from_meters(10000.1);
	println!("{0:.9} outside", t);
	println!("{0:.9} C outside", t.as_celsius());
	println!("{0:.3} away", d);
	println!("{0:.3} m away", d.as_meters());
}
