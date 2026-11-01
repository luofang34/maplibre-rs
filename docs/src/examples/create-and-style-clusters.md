# Create and Style Clusters

Group dense point data into clusters and style them with point-count thresholds.

---

## 1. Style Specification

The style specification enables clustering on a `geojson` source and uses `step` expressions to scale circle colors and radii by `point_count`:

```json
{
  "version": 8,
  "name": "MapLibre Point Clusters",
  "sources": {
    "earthquakes": {
      "type": "geojson",
      "data": "https://maplibre.org/maplibre-gl-js/docs/assets/earthquakes.geojson",
      "cluster": true,
      "clusterMaxZoom": 14,
      "clusterRadius": 50
    }
  },
  "layers": [
    {
      "id": "background",
      "type": "background",
      "paint": {
        "background-color": "#111827"
      }
    },
    {
      "id": "clusters",
      "type": "circle",
      "source": "earthquakes",
      "filter": ["has", "point_count"],
      "paint": {
        "circle-color": [
          "step",
          ["get", "point_count"],
          "#51bbd6",
          100,
          "#f1f075",
          750,
          "#f28cb1"
        ],
        "circle-radius": [
          "step",
          ["get", "point_count"],
          20,
          100,
          30,
          750,
          40
        ]
      }
    },
    {
      "id": "cluster-count",
      "type": "symbol",
      "source": "earthquakes",
      "filter": ["has", "point_count"],
      "layout": {
        "text-field": "{point_count_abbreviated}",
        "text-size": 12
      }
    },
    {
      "id": "unclustered-point",
      "type": "circle",
      "source": "earthquakes",
      "filter": ["!", ["has", "point_count"]],
      "paint": {
        "circle-color": "#11b4da",
        "circle-radius": 4,
        "circle-stroke-width": 1,
        "circle-stroke-color": "#fff"
      }
    }
  ],
  "center": [-103.59179687499999, 40.66995747011755],
  "zoom": 3.0
}
```

---

## 2. Running Natively in Rust

### Command Line

```bash
cargo run -p maplibre-demo -- headed --style path/to/cluster-style.json
```

### Native API Example

```rust
use maplibre::{
    kernel::KernelBuilder,
    map::Map,
    plugin::Plugin,
    style::Style,
};

pub fn create_cluster_map<E: maplibre::environment::Environment>(
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
  <title>MapLibre Point Clusters</title>
  <style>
    body { margin: 0; width: 100vw; height: 100vh; overflow: hidden; background: #111827; }
    #maplibre { width: 100%; height: 100%; display: block; }
  </style>
</head>
<body>
  <canvas id="maplibre"></canvas>

  <script type="module">
    import { startMapLibre } from 'maplibre-rs';

    async function init() {
      const res = await fetch('./cluster-style.json');
      const style = await res.json();
      await startMapLibre(undefined, undefined, JSON.stringify(style));
    }

    init().catch(console.error);
  </script>
</body>
</html>
```
