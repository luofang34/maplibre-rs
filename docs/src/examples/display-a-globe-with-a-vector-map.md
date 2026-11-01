# Display a Globe with a Vector Map

Display a 3D spherical globe with vector tile features using the `globe` projection.

<div class="map-container" style="position: relative; width: 100%; height: 520px; border-radius: 8px; overflow: hidden; margin: 1.5rem 0; border: 1px solid #30363d; background: #0b0e14;">
  <iframe src="./demo/index.html" style="width: 100%; height: 100%; border: none; display: block;" title="MapLibre Globe Vector Map" allow="fullscreen"></iframe>
  <div style="position: absolute; bottom: 12px; left: 12px; background: rgba(13, 17, 23, 0.85); color: #c9d1d9; padding: 6px 12px; border-radius: 6px; font-size: 12px; backdrop-filter: blur(4px); pointer-events: none; border: 1px solid rgba(255,255,255,0.1);">
    <span>Drag to orbit • Scroll to zoom • Right-click drag to pitch/rotate</span>
  </div>
</div>

---

## 1. Style Specification

The projection is declared in the root style JSON specification using the `projection` property. Setting `"type": "globe"` switches the map renderer from a flat Mercator projection to a continuous spherical globe.

```json
{
  "version": 8,
  "name": "MapLibre Globe Vector Map",
  "sources": {
    "openmaptiles": {
      "type": "vector",
      "url": "https://demotiles.maplibre.org/tiles/tiles.json"
    }
  },
  "projection": {
    "type": "globe"
  },
  "layers": [
    {
      "id": "background",
      "type": "background",
      "paint": {
        "background-color": "#1a1e24"
      }
    },
    {
      "id": "water",
      "type": "fill",
      "source": "openmaptiles",
      "source-layer": "water",
      "paint": {
        "fill-color": "#19334d"
      }
    },
    {
      "id": "land",
      "type": "fill",
      "source": "openmaptiles",
      "source-layer": "landcover",
      "paint": {
        "fill-color": "#2c3e50"
      }
    },
    {
      "id": "admin-boundaries",
      "type": "line",
      "source": "openmaptiles",
      "source-layer": "boundary",
      "paint": {
        "line-color": "#4a6984",
        "line-width": 1.0
      }
    }
  ]
}
```

---

## 2. Running Natively in Rust

### Command Line

You can run the globe map locally using the `maplibre-demo` binary:

```bash
# Run with the default vector map on the globe:
cargo run -p maplibre-demo -- headed --globe

# Or load a remote style JSON:
cargo run -p maplibre-demo -- headed --style https://demotiles.maplibre.org/style.json --projection globe
```

### Native API Example

To instantiate a globe map in a Rust application:

```rust
use std::sync::Arc;
use maplibre::{
    coords::{LatLon, WorldCoords, Zoom},
    kernel::KernelBuilder,
    map::Map,
    plugin::Plugin,
    projection::{ProjectionSpecification, ProjectionType},
    render::{
        builder::RendererBuilder,
        settings::{RendererSettings, WgpuSettings},
    },
    style::Style,
    window::PhysicalSize,
};

pub fn create_globe_map<E: maplibre::environment::Environment>(
    kernel_builder: KernelBuilder<E>,
    style_json: &str,
) -> Result<Map<E>, Box<dyn std::error::Error>> {
    let mut style: Style = serde_json::from_str(style_json)?;

    style.projection = Some(ProjectionSpecification {
        projection_type: ProjectionType::Globe,
    });

    let kernel = kernel_builder.build()?;
    let plugins: Vec<Box<dyn Plugin<E>>> = vec![
        Box::new(maplibre::render::RenderPlugin),
        Box::new(maplibre::background::BackgroundPlugin),
        Box::new(maplibre::vector::VectorPlugin::<maplibre::vector::DefaultVectorTransferables>::default()),
        Box::new(maplibre::sdf::SdfPlugin::<maplibre::vector::DefaultVectorTransferables>::default()),
    ];

    let map = Map::new(style, kernel, plugins)?;
    Ok(map)
}
```

---

## 3. WebAssembly (WASM) Deployment

MapLibre Rust compiles to WebAssembly via `wasm-bindgen` and `wgpu`, supporting WebGPU and WebGL2.

### Building the Web Distribution

```bash
# Build for WebGPU:
just web-lib build

# Build for WebGL2:
just web-lib build --webgl
```

### HTML & JavaScript Integration

```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <title>MapLibre Globe Demo</title>
  <style>
    body { margin: 0; width: 100vw; height: 100vh; overflow: hidden; background: #0b0e14; }
    #maplibre { width: 100%; height: 100%; display: block; }
  </style>
</head>
<body>
  <canvas id="maplibre"></canvas>

  <script type="module">
    import { startMapLibre } from 'maplibre-rs';

    async function init() {
      const res = await fetch('https://demotiles.maplibre.org/style.json');
      const style = await res.json();
      style.projection = { type: 'globe' };

      await startMapLibre(undefined, undefined, JSON.stringify(style));
    }

    init().catch(console.error);
  </script>
</body>
</html>
```
