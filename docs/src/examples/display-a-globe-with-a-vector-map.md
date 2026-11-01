# Display a Globe with a Vector Map

This example demonstrates how to render an interactive 3D globe with a vector map in **MapLibre Rust**, matching the behavior, visual output, and API patterns of the [MapLibre GL JS Display a Globe with a Vector Map](https://maplibre.org/maplibre-gl-js/docs/examples/display-a-globe-with-a-vector-map/) example.

You will learn how to:
1. Configure the globe projection in the MapLibre style JSON specification.
2. Run the interactive globe vector map natively using modern, idiomatic Rust.
3. Deploy and embed the map on the web using WebAssembly (WASM) with WebGPU and WebGL2.

---

## 1. Style Specification

In MapLibre, the projection is declared as a root property of the style JSON. Setting `"type": "globe"` (or `"type": "vertical-perspective"`) switches the map renderer from planar Web Mercator to a 3D spherical globe with continuous level-of-detail (LOD) tile covering.

Below is a complete style document using the public [MapLibre Demo Tiles](https://demotiles.maplibre.org/):

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

> **Zoom Transition:** As in MapLibre GL JS, the globe projection provides natural perspective curvature at global zoom levels ($z < 12$) and seamlessly transitions to standard planar resolution as the camera approaches surface features ($z \ge 12$).

---

## 2. Running Natively in Rust

### Quick Start via CLI

You can run the interactive globe vector map immediately on macOS, Linux, or Windows via the included `maplibre-demo` package:

```bash
# Run with the default MapLibre demo style on the 3D globe:
cargo run -p maplibre-demo -- headed --globe

# Or load a remote style URL directly:
cargo run -p maplibre-demo -- headed --style https://demotiles.maplibre.org/style.json --projection globe
```

### Controls:
- **Left click + drag:** Pan / rotate around the globe.
- **Scroll wheel or `+` / `-`:** Zoom in / out relative to the pointer.
- **Right click + drag:** Adjust pitch and bearing.
- **`W` / `A` / `S` / `D` or arrow keys:** Orbit and translate view center.

---

### Idiomatic Rust Integration

When integrating `maplibre` into your own Rust application or windowing loop (using `winit` or a custom event loop), construct your `Map` and `Style` as follows:

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

/// Initializes and configures an interactive globe map.
pub fn create_globe_map<E: maplibre::environment::Environment>(
    kernel_builder: KernelBuilder<E>,
    style_json: &str,
) -> Result<Map<E>, Box<dyn std::error::Error>> {
    // 1. Parse the style specification
    let mut style: Style = serde_json::from_str(style_json)?;

    // 2. Ensure the projection is set to Globe
    style.projection = Some(ProjectionSpecification {
        projection_type: ProjectionType::Globe,
    });

    // 3. Configure the kernel and instantiate plugins
    let kernel = kernel_builder.build()?;
    let plugins: Vec<Box<dyn Plugin<E>>> = vec![
        Box::new(maplibre::render::RenderPlugin),
        Box::new(maplibre::background::BackgroundPlugin),
        Box::new(maplibre::vector::VectorPlugin::<maplibre::vector::DefaultVectorTransferables>::default()),
        Box::new(maplibre::sdf::SdfPlugin::<maplibre::vector::DefaultVectorTransferables>::default()),
    ];

    // 4. Create the Map instance
    let map = Map::new(style, kernel, plugins)?;
    Ok(map)
}
```

---

## 3. WebAssembly (WASM) Deployment & Embedding

MapLibre Rust compiles directly to WebAssembly using `wasm-bindgen` and `wgpu`, supporting both **WebGPU** (for maximum performance and compute capability) and **WebGL2** (for universal browser compatibility).

### Building the WASM Library

From the repository root, use `just` to build the web distribution:

```bash
# Build for WebGPU (modern browsers):
just web-lib build

# Build for WebGL2 with multithreaded worker support:
just web-lib build --webgl --multithreaded
```

The build produces an npm-compatible package in `web/lib/dist` that can be imported directly into any web application or bundler (Vite, Webpack, ESBuild).

---

### TypeScript / JavaScript Integration

To mount the interactive globe map on an HTML5 `<canvas>` element:

```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <title>MapLibre Rust Globe Demo</title>
  <style>
    html, body {
      margin: 0;
      padding: 0;
      width: 100%;
      height: 100%;
      overflow: hidden;
      background-color: #0b0e14;
    }
    #maplibre-canvas {
      width: 100%;
      height: 100%;
      display: block;
    }
  </style>
</head>
<body>
  <canvas id="maplibre-canvas"></canvas>

  <script type="module">
    import { startMapLibre } from 'maplibre-rs';

    async function initGlobe() {
      // 1. Fetch vector style
      const response = await fetch('https://demotiles.maplibre.org/style.json');
      const style = await response.json();

      // 2. Set projection to globe
      style.projection = { type: 'globe' };

      // 3. Initialize the WebAssembly runtime on the canvas
      await startMapLibre(
        undefined,                  // Default wasm binary path
        undefined,                  // Default worker pool path
        JSON.stringify(style)       // Globe style JSON
      );
    }

    initGlobe().catch(console.error);
  </script>
</body>
</html>
```

---

### Embedded WASM Deployment in mdBook

You can embed the live interactive globe directly into documentation pages and web books.

```html
<div class="map-container" style="position: relative; width: 100%; height: 520px; border-radius: 8px; overflow: hidden; box-shadow: 0 4px 16px rgba(0,0,0,0.3); margin: 20px 0; background: #111;">
  <canvas id="maplibre" style="width: 100%; height: 100%; display: block;"></canvas>
  <div style="position: absolute; bottom: 10px; right: 10px; background: rgba(0,0,0,0.65); color: #fff; padding: 4px 8px; border-radius: 4px; font-size: 12px; pointer-events: none;">
    MapLibre Rust (WASM / WebGPU)
  </div>
</div>
```

---

## 4. Key Architectural & Performance Advantages

MapLibre Rust's globe engine includes several structural optimizations over both JavaScript and legacy C++ implementations:

1. **Zero-Allocation Subdivision**:
   During Sutherland-Hodgman polygon clipping against spherical tile boundaries, MapLibre Rust uses stack-allocated `InlinePolygon` buffers and `GlobeTileMeshCache` scratch memory. This guarantees **zero heap allocations** per tile subdivision during camera motion.

2. **Smooth Altitude-Dependent Atmosphere Shading**:
   The atmospheric limb scattering dynamically evaluates a $C^1$-continuous Hermite fade curve ($t^2(3-2t)$) from camera altitude ($< 100\text{ km}$), matching MapLibre GL JS visual fidelity without ground popping.

3. **Sub-Millimeter Floating-Point Precision**:
   At street and runway zoom levels ($z \ge 12$), large Earth-radius coordinates are resolved in double precision (`f64`) relative to the tile center before casting offsets to GPU 32-bit floats. This completely eliminates the vertex jitter commonly seen on high-zoom runways and taxiways in 32-bit engines.
