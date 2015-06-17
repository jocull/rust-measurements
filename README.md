![Build Status](https://travis-ci.org/jocull/rust-measurements.svg)

# And Rust said, "Let there be units".

Hurray! Now you can work with units of measure in a headache-free way.

Currently only `Length` is available, but I am planning to add more shortly.

### Example

In your Cargo.toml...

```
[dependencies]
measurements = "^0.1.0"
```

In your code...

```
extern crate measurements;

use measurements::length::Length;

let football_field = Length::from_yards(100.0);
let meters = football_field.as_meters();
println!("There are {} meters in a football field.", meters);
```
