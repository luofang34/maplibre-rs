# Display a Globe with a Vector Map

The `globe` projection draws the vector world on a sphere at low zoom and blends into Mercator as the camera zooms in.

<div class="map-frame">
  <iframe src="./demo/index.html?style=./styles/display-a-globe-with-a-vector-map.json" title="Display a Globe with a Vector Map" allow="fullscreen" loading="lazy"></iframe>
</div>

Data: MapLibre demotiles. GL JS counterpart: [Display a Globe with a Vector Map](https://maplibre.org/maplibre-gl-js/docs/examples/display-a-globe-with-a-vector-map/).

## Run it natively

```bash
cargo run -p maplibre-demo -- headed --style docs/src/examples/demo/styles/display-a-globe-with-a-vector-map.json
```

## Style

<details>
<summary>Style JSON</summary>

```json
{{#include demo/styles/display-a-globe-with-a-vector-map.json}}
```

</details>
