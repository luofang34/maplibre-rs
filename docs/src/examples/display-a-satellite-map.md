# Display a Satellite Map

A single `raster` source and layer.

<div class="map-frame">
  <iframe src="./demo/index.html?style=./styles/display-a-satellite-map.json" title="Display a Satellite Map" allow="fullscreen" loading="lazy"></iframe>
</div>

Data: Sentinel-2 cloudless by EOX. GL JS counterpart: [Display a Satellite Map](https://maplibre.org/maplibre-gl-js/docs/examples/display-a-satellite-map/).

## Run it natively

```bash
cargo run -p maplibre-demo -- headed --style docs/src/examples/demo/styles/display-a-satellite-map.json
```

## Style

<details>
<summary>Style JSON</summary>

```json
{{#include demo/styles/display-a-satellite-map.json}}
```

</details>
