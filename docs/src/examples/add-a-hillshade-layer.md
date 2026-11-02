# Add a Hillshade Layer

Slope shading computed in the shader from a `raster-dem` source.

<div class="map-frame">
  <iframe src="./demo/index.html?style=./styles/add-a-hillshade-layer.json" title="Add a Hillshade Layer" allow="fullscreen" loading="lazy"></iframe>
</div>

Data: Esri World Street Map, MapLibre terrain tiles. GL JS counterpart: [Add a Hillshade Layer](https://maplibre.org/maplibre-gl-js/docs/examples/add-a-hillshade-layer/).

## Run it natively

```bash
cargo run -p maplibre-demo -- headed --style docs/src/examples/demo/styles/add-a-hillshade-layer.json
```

## Style

<details>
<summary>Style JSON</summary>

```json
{{#include demo/styles/add-a-hillshade-layer.json}}
```

</details>
