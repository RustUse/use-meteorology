# use-atmospheric-pressure

Primitive atmospheric pressure vocabulary.

`use-atmospheric-pressure` stores atmospheric and sea-level pressure values as hPa or mbar `f64` wrappers, along with simple tendency and unit-label vocabulary. It does not model pressure fields, gradients, or weather prediction logic.

```rust
use use_atmospheric_pressure::{AtmosphericPressure, PressureTendency, PressureUnitLabel};

let pressure = AtmosphericPressure::new(1013.2).unwrap();

assert_eq!(pressure.hectopascals(), 1013.2);
assert_eq!(pressure.unit_label(), PressureUnitLabel::Hectopascal);
assert_eq!(PressureTendency::Rising.to_string(), "rising");
```
