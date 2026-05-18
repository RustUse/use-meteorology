# use-cloud

Primitive cloud vocabulary.

`use-cloud` models cloud kinds, cloud cover, cloud base, and simple cloud-layer composition. `CloudCover` stores oktas in `0..=8` for the initial stable convention. The crate does not classify clouds from images, process satellite data, or forecast cloud cover.

```rust
use use_cloud::{CloudBase, CloudCover, CloudKind, CloudLayer};

let layer = CloudLayer::new(CloudKind::Stratocumulus, CloudCover::new(6).unwrap())
    .with_base(CloudBase::new(900.0).unwrap());

assert_eq!(layer.kind(), &CloudKind::Stratocumulus);
assert_eq!(layer.cover().oktas(), 6);
assert_eq!(layer.base().unwrap().meters_agl(), 900.0);
```
