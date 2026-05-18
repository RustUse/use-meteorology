# use-weather-forecast

Primitive forecast vocabulary.

`use-weather-forecast` models forecast identifiers, forecast kinds, forecast confidence labels, forecast horizons, and descriptive forecast periods. It does not generate forecasts, fetch forecast data, score forecast skill, or implement alerting systems.

```rust
use use_weather_forecast::{ForecastConfidence, ForecastHorizon, ForecastId, ForecastKind};

let identifier = ForecastId::new("short-range-001").unwrap();
let horizon = ForecastHorizon::new(24);

assert_eq!(identifier.as_str(), "short-range-001");
assert_eq!(horizon.hours(), 24);
assert_eq!(ForecastKind::ShortRange.to_string(), "short-range");
assert_eq!(ForecastConfidence::High.to_string(), "high");
```
