# Display a Map

Initialize and display a standard vector map in Mercator projection.

<div class="map-container" style="position: relative; width: 100%; height: 520px; border-radius: 8px; overflow: hidden; margin: 1.5rem 0; border: 1px solid #30363d; background: #0b0e14;">
  <iframe src="./demo/index.html?projection=mercator" style="width: 100%; height: 100%; border: none; display: block;" title="MapLibre Vector Map" allow="fullscreen"></iframe>
  <div style="position: absolute; bottom: 12px; left: 12px; background: rgba(13, 17, 23, 0.85); color: #c9d1d9; padding: 6px 12px; border-radius: 6px; font-size: 12px; backdrop-filter: blur(4px); pointer-events: none; border: 1px solid rgba(255,255,255,0.1);">
    <span>Drag to pan • Scroll to zoom • Right-click drag to pitch/rotate</span>
  </div>
</div>

---

## 1. Style Specification

The style specification defines vector tile sources, background colors, and vector layers:

```json
{
  "version": 8,
  "name": "MapLibre Vector Map",
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
        "background-color": "#d8e8f8"
      }
    },
    {
      "id": "water",
      "type": "fill",
      "source": "openmaptiles",
      "source-layer": "water",
      "paint": {
        "fill-color": "#73b2ff"
      }
    },
    {
      "id": "land",
      "type": "fill",
      "source": "openmaptiles",
      "source-layer": "landcover",
      "paint": {
        "fill-color": "#e0ebd8"
      }
    },
    {
      "id": "boundaries",
      "type": "line",
      "source": "openmaptiles",
      "source-layer": "boundary",
      "paint": {
        "line-color": "#9ca7b5",
        "line-width": 1.0
      }
    }
  ],
  "center": [0.0, 20.0],
  "zoom": 1.5
}
```

---

## 2. Running Natively in Rust

### Command Line

```bash
# Run the default headed vector map:
cargo run -p maplibre-demo -- headed

# Or run with a custom remote or local style JSON:
cargo run -p maplibre-demo -- headed --style https://demotiles.maplibre.org/style.json
```

### Native API Example

```rust
use maplibre::{
    kernel::KernelBuilder,
    map::Map,
    plugin::Plugin,
    style::Style,
};

pub fn create_map<E: maplibre::environment::Environment>(
    kernel_builder: KernelBuilder<E>,
    style_json: &str,
) -> Result<Map<E>, Box<dyn std::error::Error>> {
    let style: Style = serde_json::from_str(style_json)?;
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

### HTML & JavaScript Integration

```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <title>MapLibre Vector Map</title>
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
      const res = await fetch('https://demotiles.maplibre.org/style.json');
      const style = await res.json();
      await startMapLibre(undefined, undefined, JSON.stringify(style));
    }

    init().catch(console.error);
  </script>
</body>
</html>
```
