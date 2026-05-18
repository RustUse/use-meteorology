# use-air-temperature

Primitive meteorological air temperature vocabulary.

`use-air-temperature` stores air temperature, dew point, heat index, and wind chill values as Celsius `f64` wrappers. It does not provide a unit-conversion framework, forecast logic, or heat-index and wind-chill formulas.

```rust
use use_air_temperature::{AirTemperature, DewPoint, TemperatureKind};

let air = AirTemperature::new(18.5).unwrap();
let dew_point = DewPoint::new(11.0).unwrap();

assert_eq!(air.celsius(), 18.5);
assert_eq!(dew_point.celsius(), 11.0);
assert_eq!(TemperatureKind::DewPoint.to_string(), "dew-point");
```
