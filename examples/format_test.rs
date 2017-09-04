extern crate measurements;
use measurements::Temperature;
use measurements::Length;
use measurements::Pressure;
use measurements::Volume;
use measurements::Weight;
use measurements::Speed;
fn main() {
    for power in -9..9 {
        let val: f64 = 123.456 * (10.0f64.powf(power as f64));
        println!("10^{}...", power);
        println!("Temp of {0:.3} outside", Temperature::from_kelvin(val));
        println!("Distance of {0:.3}", Length::from_meters(val));
        println!("Pressure of {0:.3}", Pressure::from_millibars(val));
        println!("Volume of {0:.3}", Volume::from_litres(val));
        println!("Weight of {0:.3}", Weight::from_kilograms(val));
        println!("Speed of {0:.3}", Speed::from_meters_per_second(val));
    }
}
