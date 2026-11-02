# Visualize Population Density

`let`, `var` and nested `interpolate` expressions colour each province by people per square kilometre.

<div class="map-frame">
  <iframe src="./demo/index.html?style=./styles/visualize-population-density.json" title="Visualize Population Density" allow="fullscreen" loading="lazy"></iframe>
</div>

Data: OpenFreeMap, OpenStreetMap, Rwanda provinces. GL JS counterpart: [Visualize Population Density](https://maplibre.org/maplibre-gl-js/docs/examples/visualize-population-density/).

## Run it natively

```bash
cargo run -p maplibre-demo -- headed --style docs/src/examples/demo/styles/visualize-population-density.json
```

## Style

<details>
<summary>Style JSON</summary>

```json
{{#include demo/styles/visualize-population-density.json}}
```

</details>
