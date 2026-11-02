# Add a Color Relief Layer

A `color-relief` layer maps the `["elevation"]` expression to a colour ramp.

<div class="map-frame">
  <iframe src="./demo/index.html?style=./styles/add-a-color-relief-layer.json" title="Add a Color Relief Layer" allow="fullscreen" loading="lazy"></iframe>
</div>

Data: Esri World Street Map, MapLibre terrain tiles. GL JS counterpart: [Add a Color Relief Layer](https://maplibre.org/maplibre-gl-js/docs/examples/add-a-color-relief-layer/).

## Run it natively

```bash
cargo run -p maplibre-demo -- headed --style docs/src/examples/demo/styles/add-a-color-relief-layer.json
```

## Style

<details>
<summary>Style JSON</summary>

```json
{{#include demo/styles/add-a-color-relief-layer.json}}
```

</details>
