extern crate measurements;
use measurements::Temperature;
use measurements::Length;
use measurements::Pressure;
use measurements::Volume;
use measurements::Weight;
use measurements::Speed;
use measurements::Acceleration;
use measurements::Energy;

fn main() {
    for power in -12..12 {
        let val: f64 = 123.456 * (10f64.powf(power as f64));
        println!("10^{}...", power);
        println!("Temp of {0:.3} outside", Temperature::from_kelvin(val));
        println!("Distance of {0:.3}", Length::from_meters(val));
        println!("Pressure of {0:.3}", Pressure::from_millibars(val));
        println!("Volume of {0:.3}", Volume::from_litres(val));
        println!("Weight of {0:.3}", Weight::from_kilograms(val));
        println!("Speed of {0:.3}", Speed::from_meters_per_second(val));
        println!("Acceleration of {0:.3}", Acceleration::from_meters_per_second_per_second(val));
        println!("Energy of {0:.3}", Energy::from_joules(val));
    }

    println!("1.0 GeV is {}", Energy::from_e_v(1e9));
}
