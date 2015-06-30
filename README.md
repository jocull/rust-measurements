[![Build Status](https://travis-ci.org/jocull/rust-measurements.svg)](https://travis-ci.org/jocull/rust-measurements)

# And Rust said, "Let there be units".

Hurray! Now you can work with units of measure in a headache-free way.

Currently available units:

- Length
- Temperature
- Weight

### Example

In your Cargo.toml...

```
[dependencies]
measurements = "^0.2.0"
```

In your code...

```rust
extern crate measurements;

use measurements::{Length, Temperature, Weight};

let football_field = Length::from_yards(100.0);
let meters = football_field.as_meters();
println!("There are {} meters in a football field.", meters);

let boiling_water = Temperature::from_celsius(100.0);
let fahrenheit = boiling_water.as_fahrenheit();
println!("Boiling water measures at {} degrees fahrenheit.", fahrenheit);

let metric_ton = Weight::from_metric_tons(1.0);
let united_states_tons = metric_ton.as_short_tons();
let united_states_pounds = metric_ton.as_pounds();
println!("One metric ton is {} U.S. tons - that's {} pounds!", united_states_tons, united_states_pounds);
```
