# Display a Map with MLT

MapLibre Tiles decode directly into the renderer; the tile format is detected from the bytes.

<div class="map-frame">
  <iframe src="./demo/index.html?style=./styles/display-a-map-with-mlt.json" title="Display a Map with MLT" allow="fullscreen" loading="lazy"></iframe>
</div>

Data: MapLibre demotiles. GL JS counterpart: [Display a Map with MLT](https://maplibre.org/maplibre-gl-js/docs/examples/display-a-map-with-mlt/).

## Run it natively

```bash
cargo run -p maplibre-demo -- headed --style docs/src/examples/demo/styles/display-a-map-with-mlt.json
```

## Style

<details>
<summary>Style JSON</summary>

```json
{{#include demo/styles/display-a-map-with-mlt.json}}
```

</details>
