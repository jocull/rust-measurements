extern crate measurements;
use measurements::Temperature;
fn main() {
	let t = Temperature::from_celsius(0.1);
	println!("{0:.9} outside", t);
	println!("{0:.9} outside", t.as_celsius());
}
