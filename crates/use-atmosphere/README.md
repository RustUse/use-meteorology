# use-atmosphere

Primitive atmosphere vocabulary.

`use-atmosphere` models atmosphere layers, descriptive atmospheric conditions, air masses, and visibility conditions. It does not simulate atmospheric layers, model circulation, or calculate atmospheric movement.

```rust
use use_atmosphere::{AirMassKind, AtmosphericCondition, AtmosphereLayer, VisibilityCondition};

let condition = AtmosphericCondition::new("stable boundary layer").unwrap();

assert_eq!(AtmosphereLayer::Troposphere.to_string(), "troposphere");
assert_eq!(AirMassKind::MaritimePolar.to_string(), "maritime-polar");
assert_eq!(VisibilityCondition::Mist.to_string(), "mist");
assert_eq!(condition.as_str(), "stable boundary layer");
```
