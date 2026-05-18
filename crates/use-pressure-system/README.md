# use-pressure-system

Primitive pressure system vocabulary.

`use-pressure-system` models pressure system kinds, pressure system names, simple strength labels, and cyclone kind vocabulary. It does not model pressure fields, track storms, or fetch advisories.

```rust
use use_pressure_system::{CycloneKind, PressureSystemKind, PressureSystemName};

let name = PressureSystemName::new("Azores High").unwrap();

assert_eq!(name.as_str(), "Azores High");
assert_eq!(PressureSystemKind::High.to_string(), "high");
assert_eq!(CycloneKind::PolarLow.to_string(), "polar-low");
```
