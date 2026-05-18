# use-weather-observation

Primitive weather observation vocabulary.

`use-weather-observation` models observation identifiers, observation kinds, descriptive sources, and observation quality labels. It does not fetch observations, process station feeds, stream sensor data, or manage weather station runtimes.

```rust
use use_weather_observation::{
    ObservationKind, ObservationQuality, ObservationSource, WeatherObservation,
    WeatherObservationId,
};

let observation = WeatherObservation::new(
    WeatherObservationId::new("obs-001").unwrap(),
    ObservationKind::Surface,
    ObservationSource::new("manual station").unwrap(),
    ObservationQuality::Verified,
);

assert_eq!(observation.id().as_str(), "obs-001");
assert_eq!(observation.kind(), &ObservationKind::Surface);
assert_eq!(observation.source().as_str(), "manual station");
assert_eq!(observation.quality(), &ObservationQuality::Verified);
```
