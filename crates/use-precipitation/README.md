# use-precipitation

Primitive precipitation vocabulary.

`use-precipitation` models precipitation kinds, intensity labels, amounts, and rates. It does not process radar data, forecast precipitation, or implement hydrology and alerting logic.

```rust
use use_precipitation::{PrecipitationAmount, PrecipitationIntensity, PrecipitationKind};

let amount = PrecipitationAmount::new(6.5).unwrap();

assert_eq!(amount.millimeters(), 6.5);
assert_eq!(PrecipitationKind::Snow.to_string(), "snow");
assert_eq!(PrecipitationIntensity::Light.to_string(), "light");
```
