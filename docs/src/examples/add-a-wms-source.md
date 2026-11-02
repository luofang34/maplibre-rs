# Add a WMS Source

The `{bbox-epsg-3857}` token asks a WMS server for each tile.

<div class="map-frame">
  <iframe src="./demo/index.html?style=./styles/add-a-wms-source.json" title="Add a WMS Source" allow="fullscreen" loading="lazy"></iframe>
</div>

Data: terrestris OSM WMS. GL JS counterpart: [Add a WMS Source](https://maplibre.org/maplibre-gl-js/docs/examples/add-a-wms-source/).

## Run it natively

```bash
cargo run -p maplibre-demo -- headed --style docs/src/examples/demo/styles/add-a-wms-source.json
```

## Style

<details>
<summary>Style JSON</summary>

```json
{{#include demo/styles/add-a-wms-source.json}}
```

</details>
