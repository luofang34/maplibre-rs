# Fly to a Location

`Map::context_mut` gives the `MapContext` whose `fly_to` moves the camera along a Van Wijk zoom-out curve, or eases along `FlightCurve::Ease`. `Map::jump_to` moves at once.

```rust
{{#include ../../../maplibre/tests/book_runtime_listings.rs:fly_to}}
```

GL JS counterpart: [Fly to a Location](https://maplibre.org/maplibre-gl-js/docs/examples/fly-to-a-location/).
