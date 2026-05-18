# use-meteorology

RustUse is “Composable sets of primitive Rust utility crates for fellow crustaceans.”

`use-meteorology` is a primitive meteorology vocabulary set. It provides small, composable Rust primitives for describing weather observations, atmospheric conditions, air temperature, atmospheric pressure, humidity, wind, clouds, precipitation, weather fronts, pressure systems, and forecast vocabulary.

`use-meteorology` is not a weather API client, forecasting engine, climate model, atmospheric simulation system, storm prediction system, radar processor, weather station runtime, GIS system, meteorology framework, or alerting system. It does not fetch weather data, predict weather, simulate the atmosphere, process radar or satellite feeds, manage station runtimes, or issue alerts.

## Boundary With Adjacent Sets

- `use-geography` owns location, place, region, and spatial-reference primitives.
- `use-physics` owns broader pressure, fluids, waves, radiation, and thermodynamic concepts.
- `use-units` and `use-measure` own general unit and measurement primitives.
- `use-time` owns timestamps, durations, intervals, and calendars.
- `use-data` and `use-validate` complement this set for storage-agnostic data containers and validation helpers.

`use-meteorology` stays focused on descriptive meteorology vocabulary. It should describe weather concepts, not fetch, predict, simulate, alert, or analyze them.

## Crates

- `use-meteorology`: facade crate for the full primitive vocabulary set
- `use-weather-observation`: primitive weather observation vocabulary
- `use-atmosphere`: primitive atmosphere vocabulary
- `use-air-temperature`: primitive meteorological air temperature vocabulary
- `use-atmospheric-pressure`: primitive atmospheric pressure vocabulary
- `use-humidity`: primitive humidity vocabulary
- `use-wind`: primitive wind vocabulary
- `use-cloud`: primitive cloud vocabulary
- `use-precipitation`: primitive precipitation vocabulary
- `use-weather-front`: primitive weather front vocabulary
- `use-pressure-system`: primitive pressure system vocabulary
- `use-weather-forecast`: primitive forecast vocabulary

## Scope

- small focused crates
- primitives over frameworks
- few or no dependencies
- stable APIs
- Rust 2024 edition
- strong documentation
- meaningful tests
- composable exports
- dual MIT OR Apache-2.0 license
- no unnecessary macros
- no async
- no global runtime assumptions
- no unsafe code
- no network calls
- no simulation
- no prediction engine
- no GIS behavior

## Example

```rust
use use_meteorology::prelude::{
    AirTemperature, AtmosphericPressure, CloudKind, ForecastKind, ObservationKind,
    PrecipitationKind, PressureSystemKind, RelativeHumidity, WeatherFrontKind, WindDirection,
    WindSpeed,
};

let observation_kind = ObservationKind::Surface;
let air_temperature = AirTemperature::new(21.5).unwrap();
let pressure = AtmosphericPressure::new(1012.4).unwrap();
let humidity = RelativeHumidity::new(68.0).unwrap();
let wind_speed = WindSpeed::new(7.2).unwrap();
let wind_direction = WindDirection::new(225.0).unwrap();
let cloud_kind = CloudKind::Cumulus;
let precipitation_kind = PrecipitationKind::Rain;
let front_kind = WeatherFrontKind::Cold;
let pressure_system_kind = PressureSystemKind::Low;
let forecast_kind = ForecastKind::ShortRange;

assert_eq!(observation_kind.to_string(), "surface");
assert_eq!(air_temperature.celsius(), 21.5);
assert_eq!(pressure.hectopascals(), 1012.4);
assert_eq!(humidity.percent(), 68.0);
assert_eq!(wind_speed.meters_per_second(), 7.2);
assert_eq!(wind_direction.degrees_from_north(), 225.0);
assert_eq!(cloud_kind.to_string(), "cumulus");
assert_eq!(precipitation_kind.to_string(), "rain");
assert_eq!(front_kind.to_string(), "cold");
assert_eq!(pressure_system_kind.to_string(), "low");
assert_eq!(forecast_kind.to_string(), "short-range");
```

This example describes meteorological concepts. It does not fetch weather data, predict weather, simulate atmosphere, process radar, or issue alerts.

## Related Sets

- `use-geography`
- `use-physics`
- `use-units`
- `use-measure`
- `use-time`
- `use-data`
- `use-validate`

## License

Licensed under either of the following, at your option:

- MIT License
- Apache License, Version 2.0
