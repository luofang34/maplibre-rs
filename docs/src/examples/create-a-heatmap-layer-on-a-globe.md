# Create a Heatmap Layer on a Globe

Heatmap density drawn in screen space over the sphere, using the same earthquake data as the flat heatmap.

<div class="map-frame">
  <iframe src="./demo/index.html?style=./styles/create-a-heatmap-layer-on-a-globe.json" title="Create a Heatmap Layer on a Globe" allow="fullscreen" loading="lazy"></iframe>
</div>

Data: MapLibre demotiles, USGS earthquakes. GL JS counterpart: [Create a Heatmap Layer on a Globe](https://maplibre.org/maplibre-gl-js/docs/examples/create-a-heatmap-layer-on-a-globe-with-terrain-elevation/).

## Run it natively

```bash
cargo run -p maplibre-demo -- headed --style docs/src/examples/demo/styles/create-a-heatmap-layer-on-a-globe.json
```

## Style

<details>
<summary>Style JSON</summary>

```json
{{#include demo/styles/create-a-heatmap-layer-on-a-globe.json}}
```

</details>
