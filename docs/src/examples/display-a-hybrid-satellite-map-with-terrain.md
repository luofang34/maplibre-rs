# Display a Hybrid Satellite Map with Terrain

Satellite imagery, 3D terrain and vector labels from different sources in one style.

<div class="map-frame">
  <iframe src="./demo/index.html?style=./styles/display-a-hybrid-satellite-map-with-terrain.json" title="Display a Hybrid Satellite Map with Terrain" allow="fullscreen" loading="lazy"></iframe>
</div>

Data: Esri World Imagery, OpenFreeMap, OpenStreetMap, MapLibre terrain tiles. GL JS counterpart: [Display a Hybrid Satellite Map with Terrain](https://maplibre.org/maplibre-gl-js/docs/examples/display-a-hybrid-satellite-map-with-terrain-elevation/).

## Run it natively

```bash
cargo run -p maplibre-demo -- headed --style docs/src/examples/demo/styles/display-a-hybrid-satellite-map-with-terrain.json --max-pitch 85
```

## Style

<details>
<summary>Style JSON</summary>

```json
{{#include demo/styles/display-a-hybrid-satellite-map-with-terrain.json}}
```

</details>
