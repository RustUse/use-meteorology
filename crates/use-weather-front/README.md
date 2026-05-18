# use-weather-front

Primitive weather front vocabulary.

`use-weather-front` models front kinds, movement labels, and strength labels. It does not model front geometry, front position, or front forecast logic.

```rust
use use_weather_front::{FrontMovement, FrontStrength, WeatherFrontKind};

assert_eq!(WeatherFrontKind::Cold.to_string(), "cold");
assert_eq!(FrontMovement::Advancing.to_string(), "advancing");
assert_eq!(FrontStrength::Strong.to_string(), "strong");
```
