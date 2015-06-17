# And Rust said, "Let there be units."

Hurray! Now you can work with units of measure in a headache-free way.

Currently only `Length` is available, but I am planning to add more shortly.

### Example

```
let football_field = Length::from_yards(1.0);
let meters = football_field.as_meters();
println!("There are {} meters in a football field.", meters);
```
