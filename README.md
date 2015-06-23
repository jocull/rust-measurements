[![Build Status](https://travis-ci.org/jocull/rust-measurements.svg)](https://travis-ci.org/jocull/rust-measurements)

# And Rust said, "Let there be units".

Hurray! Now you can work with units of measure in a headache-free way.

Currently available units:

- Length
- Temperature

### Example

In your Cargo.toml...

```
[dependencies]
measurements = "^0.2.0"
```

In your code...

```
extern crate measurements;

use measurements::length::Length;
use measurements::temperature::Temperature;

let football_field = Length::from_yards(100.0);
let meters = football_field.as_meters();
println!("There are {} meters in a football field.", meters);

let boiling_water = Temperature::from_celsius(100.0);
let fahrenheit = boiling_water.as_fahrenheit();
println!("Boiling water measures at {} degrees fahrenheit.", fahrenheit);
```
