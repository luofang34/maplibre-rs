# Display a Map

A vector basemap in Mercator from one style URL.

<div class="map-frame">
  <iframe src="./demo/index.html?style=./styles/display-a-map.json" title="Display a Map" allow="fullscreen" loading="lazy"></iframe>
</div>

Data: MapLibre demotiles. GL JS counterpart: [Display a Map](https://maplibre.org/maplibre-gl-js/docs/examples/display-a-map/).

## Run it natively

```bash
cargo run -p maplibre-demo -- headed --style docs/src/examples/demo/styles/display-a-map.json
```

## Style

<details>
<summary>Style JSON</summary>

```json
{{#include demo/styles/display-a-map.json}}
```

</details>
