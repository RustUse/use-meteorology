# use-wind

Primitive wind vocabulary.

`use-wind` models wind speed, wind direction, gusts, stable wind kind labels, and Beaufort scale values. It does not forecast wind, model vector fields, or process radar or station feeds.

```rust
use use_wind::{BeaufortScale, WindDirection, WindKind, WindSpeed};

let speed = WindSpeed::new(12.0).unwrap();
let direction = WindDirection::new(270.0).unwrap();
let beaufort = BeaufortScale::new(6).unwrap();

assert_eq!(speed.meters_per_second(), 12.0);
assert_eq!(direction.degrees_from_north(), 270.0);
assert_eq!(beaufort.value(), 6);
assert_eq!(WindKind::Gale.to_string(), "gale");
```
