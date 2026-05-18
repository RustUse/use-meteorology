# use-meteorology

Facade crate for primitive RustUse meteorology vocabulary.

`use-meteorology` re-exports focused crates for weather observations, atmosphere, air temperature, atmospheric pressure, humidity, wind, clouds, precipitation, weather fronts, pressure systems, and forecast vocabulary.

`use-meteorology` is not a weather API client, forecasting engine, climate model, atmospheric simulator, radar processor, weather station runtime, or meteorology framework. It describes meteorological concepts; it does not fetch, predict, simulate, process feeds, or issue alerts.

```rust
use use_meteorology::prelude::{
    AirTemperature, AtmosphericPressure, CloudKind, ForecastKind, ObservationKind,
    PrecipitationKind, PressureSystemKind, RelativeHumidity, WeatherFrontKind, WindDirection,
    WindSpeed,
};

let observation_kind = ObservationKind::Surface;
let air_temperature = AirTemperature::new(19.0).unwrap();
let pressure = AtmosphericPressure::new(1009.8).unwrap();
let humidity = RelativeHumidity::new(63.0).unwrap();
let wind_speed = WindSpeed::new(5.5).unwrap();
let wind_direction = WindDirection::new(180.0).unwrap();
let cloud_kind = CloudKind::Cumulus;
let precipitation_kind = PrecipitationKind::Drizzle;
let front_kind = WeatherFrontKind::Warm;
let pressure_system_kind = PressureSystemKind::Low;
let forecast_kind = ForecastKind::Nowcast;

assert_eq!(observation_kind.to_string(), "surface");
assert_eq!(air_temperature.celsius(), 19.0);
assert_eq!(pressure.hectopascals(), 1009.8);
assert_eq!(humidity.percent(), 63.0);
assert_eq!(wind_speed.meters_per_second(), 5.5);
assert_eq!(wind_direction.degrees_from_north(), 180.0);
assert_eq!(cloud_kind.to_string(), "cumulus");
assert_eq!(precipitation_kind.to_string(), "drizzle");
assert_eq!(front_kind.to_string(), "warm");
assert_eq!(pressure_system_kind.to_string(), "low");
assert_eq!(forecast_kind.to_string(), "nowcast");
```

This crate composes primitive meteorology vocabulary. It does not fetch weather data, generate forecasts, simulate the atmosphere, process radar or satellite products, or issue alerts.
