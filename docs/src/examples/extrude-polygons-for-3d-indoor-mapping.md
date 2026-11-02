# Extrude Polygons for 3D Indoor Mapping

Height, base and colour of every room come from GeoJSON properties.

<div class="map-frame">
  <iframe src="./demo/index.html?style=./styles/extrude-polygons-for-3d-indoor-mapping.json" title="Extrude Polygons for 3D Indoor Mapping" allow="fullscreen" loading="lazy"></iframe>
</div>

Data: Esri World Street Map, GL JS indoor sample data. GL JS counterpart: [Extrude Polygons for 3D Indoor Mapping](https://maplibre.org/maplibre-gl-js/docs/examples/extrude-polygons-for-3d-indoor-mapping/).

## Run it natively

```bash
cargo run -p maplibre-demo -- headed --style docs/src/examples/demo/styles/extrude-polygons-for-3d-indoor-mapping.json
```

## Style

<details>
<summary>Style JSON</summary>

```json
{{#include demo/styles/extrude-polygons-for-3d-indoor-mapping.json}}
```

</details>
