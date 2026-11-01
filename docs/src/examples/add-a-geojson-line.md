# Add a GeoJSON Line

Add a GeoJSON line to the map and style it with color, width, and join properties.

---

## 1. Style Specification

The style specification defines an inline or remote `geojson` source with a `LineString` feature and a corresponding `line` layer:

```json
{
  "version": 8,
  "name": "MapLibre GeoJSON Line",
  "sources": {
    "route": {
      "type": "geojson",
      "data": {
        "type": "Feature",
        "properties": {},
        "geometry": {
          "type": "LineString",
          "coordinates": [
            [-122.483696, 37.833818],
            [-122.483482, 37.833174],
            [-122.483396, 37.8327],
            [-122.483568, 37.832056],
            [-122.48404, 37.831141],
            [-122.48404, 37.830497],
            [-122.48346, 37.82992],
            [-122.483568, 37.829548],
            [-122.48507, 37.829446],
            [-122.4861, 37.828802],
            [-122.486958, 37.82831],
            [-122.487859, 37.825522],
            [-122.488546, 37.824028],
            [-122.488031, 37.823384],
            [-122.48713, 37.823516],
            [-122.486636, 37.824681],
            [-122.486604, 37.8252],
            [-122.487076, 37.8258]
          ]
        }
      }
    }
  },
  "layers": [
    {
      "id": "background",
      "type": "background",
      "paint": {
        "background-color": "#e0e4eb"
      }
    },
    {
      "id": "route-line",
      "type": "line",
      "source": "route",
      "layout": {
        "line-join": "round",
        "line-cap": "round"
      },
      "paint": {
        "line-color": "#1db7dd",
        "line-width": 8.0
      }
    }
  ],
  "center": [-122.486052, 37.830348],
  "zoom": 14.0
}
```

---

## 2. Running Natively in Rust

### Command Line

```bash
cargo run -p maplibre-demo -- headed --style path/to/geojson-line-style.json
```

### Native API Example

```rust
use maplibre::{
    kernel::KernelBuilder,
    map::Map,
    plugin::Plugin,
    style::Style,
};

pub fn create_geojson_map<E: maplibre::environment::Environment>(
    kernel_builder: KernelBuilder<E>,
    style_json: &str,
) -> Result<Map<E>, Box<dyn std::error::Error>> {
    let style: Style = serde_json::from_str(style_json)?;
    let kernel = kernel_builder.build()?;
    let plugins: Vec<Box<dyn Plugin<E>>> = vec![
        Box::new(maplibre::render::RenderPlugin),
        Box::new(maplibre::background::BackgroundPlugin),
        Box::new(maplibre::vector::VectorPlugin::<maplibre::vector::DefaultVectorTransferables>::default()),
    ];

    let map = Map::new(style, kernel, plugins)?;
    Ok(map)
}
```

---

## 3. WebAssembly (WASM) Deployment

### HTML & JavaScript Integration

```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <title>MapLibre GeoJSON Line</title>
  <style>
    body { margin: 0; width: 100vw; height: 100vh; overflow: hidden; }
    #maplibre { width: 100%; height: 100%; display: block; }
  </style>
</head>
<body>
  <canvas id="maplibre"></canvas>

  <script type="module">
    import { startMapLibre } from 'maplibre-rs';

    async function init() {
      const res = await fetch('./geojson-line-style.json');
      const style = await res.json();
      await startMapLibre(undefined, undefined, JSON.stringify(style));
    }

    init().catch(console.error);
  </script>
</body>
</html>
```
