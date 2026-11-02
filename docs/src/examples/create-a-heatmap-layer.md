# Create a Heatmap Layer

A `heatmap` layer that gives way to individual `circle` points as the camera zooms in.

<div class="map-frame">
  <iframe src="./demo/index.html?style=./styles/create-a-heatmap-layer.json" title="Create a Heatmap Layer" allow="fullscreen" loading="lazy"></iframe>
</div>

Data: MapLibre demotiles, USGS earthquakes. GL JS counterpart: [Create a Heatmap Layer](https://maplibre.org/maplibre-gl-js/docs/examples/create-a-heatmap-layer/).

## Run it natively

```bash
cargo run -p maplibre-demo -- headed --style docs/src/examples/demo/styles/create-a-heatmap-layer.json
```

## Style

<details>
<summary>Style JSON</summary>

```json
{{#include demo/styles/create-a-heatmap-layer.json}}
```

</details>
