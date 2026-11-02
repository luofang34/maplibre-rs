# Add a Multidirectional Hillshade Layer

`hillshade-method: multidirectional` combines up to four coloured light directions.

<div class="map-frame">
  <iframe src="./demo/index.html?style=./styles/add-a-multidirectional-hillshade-layer.json" title="Add a Multidirectional Hillshade Layer" allow="fullscreen" loading="lazy"></iframe>
</div>

Data: Esri World Street Map, MapLibre terrain tiles. GL JS counterpart: [Add a Multidirectional Hillshade Layer](https://maplibre.org/maplibre-gl-js/docs/examples/add-a-multidirectional-hillshade-layer/).

## Run it natively

```bash
cargo run -p maplibre-demo -- headed --style docs/src/examples/demo/styles/add-a-multidirectional-hillshade-layer.json
```

## Style

<details>
<summary>Style JSON</summary>

```json
{{#include demo/styles/add-a-multidirectional-hillshade-layer.json}}
```

</details>
