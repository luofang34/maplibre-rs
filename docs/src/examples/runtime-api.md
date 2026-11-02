# Runtime API

A style describes a map when it loads. A running [`Map`](https://docs.rs/maplibre/latest/maplibre/map/struct.Map.html) changes it from Rust, with methods named after their GL JS counterparts. Hosts call them from their own event loop, or from a plugin's system.

The listings on these pages are compiled by `maplibre/tests/book_runtime_listings.rs`.

- [Fly to a Location](./fly-to-a-location.md)
- [Change Style at Runtime](./change-a-layers-paint.md)
- [Update GeoJSON Data](./update-geojson-data.md)
- [Query Rendered Features](./query-rendered-features.md)
- [Add a Source and Layer at Runtime](./add-layers-at-runtime.md)
