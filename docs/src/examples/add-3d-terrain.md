# Add 3D Terrain to a Map

Render 3D elevation and topography using digital elevation models (`raster-dem`) and camera pitch.

<div class="map-container" style="position: relative; width: 100%; height: 520px; border-radius: 8px; overflow: hidden; margin: 1.5rem 0; border: 1px solid #30363d; background: #0b0e14;">
  <iframe src="./demo/index.html?style=./terrain.json" style="width: 100%; height: 100%; border: none; display: block;" title="MapLibre 3D Terrain" allow="fullscreen"></iframe>
  <div style="position: absolute; bottom: 12px; left: 12px; background: rgba(13, 17, 23, 0.85); color: #c9d1d9; padding: 6px 12px; border-radius: 6px; font-size: 12px; backdrop-filter: blur(4px); pointer-events: none; border: 1px solid rgba(255,255,255,0.1);">
    <span>Drag to pan/orbit • Scroll to zoom • Right-click drag to pitch/rotate</span>
  </div>
</div>

---

## 1. Style Specification

The style specification configures a `raster-dem` elevation source and attaches it via the root `terrain` property:

```json
{
  "version": 8,
  "name": "MapLibre 3D Terrain",
  "sources": {
    "osm": {
      "type": "raster",
      "tiles": [
        "https://tile.openstreetmap.org/{z}/{x}/{y}.png"
      ],
      "tileSize": 256,
      "attribution": "&copy; OpenStreetMap contributors",
      "maxzoom": 19
    },
    "terrainSource": {
      "type": "raster-dem",
      "url": "https://demotiles.maplibre.org/terrain-tiles/tiles.json",
      "tileSize": 256
    }
  },
  "layers": [
    {
      "id": "background",
      "type": "background",
      "paint": {
        "background-color": "#dfe6ea"
      }
    },
    {
      "id": "osm",
      "type": "raster",
      "source": "osm"
    }
  ],
  "terrain": {
    "source": "terrainSource",
    "exaggeration": 1.0
  },
  "center": [11.39085, 47.27574],
  "zoom": 12.0,
  "pitch": 70.0
}
```

---

## 2. Running Natively in Rust

### Command Line

```bash
# Run with the bundled 3D terrain:
cargo run -p maplibre-demo -- headed --terrain

# Run with custom max-pitch up to 85 degrees:
cargo run -p maplibre-demo -- headed --terrain --max-pitch 85
```

### Native API Example

```rust
use maplibre::{
    kernel::KernelBuilder,
    map::Map,
    plugin::Plugin,
    style::Style,
    terrain::TerrainPlugin,
};

pub fn create_terrain_map<E: maplibre::environment::Environment>(
    kernel_builder: KernelBuilder<E>,
    style_json: &str,
) -> Result<Map<E>, Box<dyn std::error::Error>> {
    let style: Style = serde_json::from_str(style_json)?;
    let kernel = kernel_builder.build()?;
    let plugins: Vec<Box<dyn Plugin<E>>> = vec![
        Box::new(maplibre::render::RenderPlugin),
        Box::new(maplibre::background::BackgroundPlugin),
        Box::new(maplibre::raster::RasterPlugin::default()),
        Box::new(TerrainPlugin::default()),
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
  <title>MapLibre 3D Terrain</title>
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
      const res = await fetch('./terrain.json');
      const style = await res.json();
      await startMapLibre(undefined, undefined, JSON.stringify(style));
    }

    init().catch(console.error);
  </script>
</body>
</html>
```
