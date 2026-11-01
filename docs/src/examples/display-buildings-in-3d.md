# Display Buildings in 3D

Render 3D building models using `fill-extrusion` layers, configuring height, base offsets, and camera tilt.

---

## 1. Style Specification

The `fill-extrusion` layer type converts 2D building footprints into 3D polygonal meshes using feature attributes (such as `render_height` and `render_min_height`):

```json
{
  "version": 8,
  "name": "MapLibre 3D Buildings",
  "sources": {
    "openmaptiles": {
      "type": "vector",
      "url": "https://demotiles.maplibre.org/tiles/tiles.json"
    }
  },
  "layers": [
    {
      "id": "background",
      "type": "background",
      "paint": {
        "background-color": "#f2efe9"
      }
    },
    {
      "id": "water",
      "type": "fill",
      "source": "openmaptiles",
      "source-layer": "water",
      "paint": {
        "fill-color": "#a0c8f0"
      }
    },
    {
      "id": "3d-buildings",
      "type": "fill-extrusion",
      "source": "openmaptiles",
      "source-layer": "building",
      "paint": {
        "fill-extrusion-color": "#aaa",
        "fill-extrusion-height": [
          "interpolate",
          ["linear"],
          ["zoom"],
          15,
          0,
          15.05,
          ["get", "render_height"]
        ],
        "fill-extrusion-base": [
          "interpolate",
          ["linear"],
          ["zoom"],
          15,
          0,
          15.05,
          ["get", "render_min_height"]
        ],
        "fill-extrusion-opacity": 0.8
      }
    }
  ],
  "center": [-74.006, 40.7128],
  "zoom": 15.5,
  "pitch": 45.0,
  "bearing": -17.6
}
```

---

## 2. Running Natively in Rust

### Command Line

```bash
cargo run -p maplibre-demo -- headed --style path/to/buildings-style.json
```

### Native API Example

```rust
use maplibre::{
    kernel::KernelBuilder,
    map::Map,
    plugin::Plugin,
    style::Style,
};

pub fn create_buildings_map<E: maplibre::environment::Environment>(
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
  <title>MapLibre 3D Buildings</title>
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
      const res = await fetch('./buildings-style.json');
      const style = await res.json();
      await startMapLibre(undefined, undefined, JSON.stringify(style));
    }

    init().catch(console.error);
  </script>
</body>
</html>
```
