# Update GeoJSON Data

`set_geojson_data` replaces a source; call it per frame to animate a line or a point. Inline data is shared, so unchanged features are not copied.

```rust
{{#include ../../../maplibre/tests/book_runtime_listings.rs:geojson}}
```

GL JS counterpart: [Update GeoJSON Data](https://maplibre.org/maplibre-gl-js/docs/examples/update-a-feature-in-realtime/).
