# Add 3D Terrain to a Map

Raster imagery draped over `raster-dem` elevation through the root `terrain` object, on the globe.

<div class="map-frame">
  <iframe src="./demo/index.html?style=./styles/add-3d-terrain.json" title="Add 3D Terrain to a Map" allow="fullscreen" loading="lazy"></iframe>
</div>

Data: Esri World Imagery, MapLibre terrain tiles. GL JS counterpart: [Add 3D Terrain to a Map](https://maplibre.org/maplibre-gl-js/docs/examples/3d-terrain/).

## Run it natively

```bash
cargo run -p maplibre-demo -- headed --style docs/src/examples/demo/styles/add-3d-terrain.json --max-pitch 85
```

## Style

<details>
<summary>Style JSON</summary>

```json
{{#include demo/styles/add-3d-terrain.json}}
```

</details>
