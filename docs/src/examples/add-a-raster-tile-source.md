# Add a Raster Tile Source

A third-party raster tile service, addressed with `{z}`, `{x}` and `{y}` tokens.

<div class="map-frame">
  <iframe src="./demo/index.html?style=./styles/add-a-raster-tile-source.json" title="Add a Raster Tile Source" allow="fullscreen" loading="lazy"></iframe>
</div>

Data: Esri World Street Map. GL JS counterpart: [Add a Raster Tile Source](https://maplibre.org/maplibre-gl-js/docs/examples/add-a-raster-tile-source/).

## Run it natively

```bash
cargo run -p maplibre-demo -- headed --style docs/src/examples/demo/styles/add-a-raster-tile-source.json
```

## Style

<details>
<summary>Style JSON</summary>

```json
{{#include demo/styles/add-a-raster-tile-source.json}}
```

</details>
