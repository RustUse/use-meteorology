# use-humidity

Primitive humidity vocabulary.

`use-humidity` models relative humidity, specific humidity, and stable humidity kind labels. It does not calculate psychrometric values, vapor pressure, or thermodynamic humidity models.

```rust
use use_humidity::{HumidityKind, RelativeHumidity, SpecificHumidity};

let relative = RelativeHumidity::new(72.5).unwrap();
let specific = SpecificHumidity::new(0.009).unwrap();

assert_eq!(relative.percent(), 72.5);
assert_eq!(specific.kilograms_per_kilogram(), 0.009);
assert_eq!(HumidityKind::Relative.to_string(), "relative");
```
