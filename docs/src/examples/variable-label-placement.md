# Variable Label Placement

`text-variable-anchor` tries several anchors and collision detection keeps the ones that fit.

<div class="map-frame">
  <iframe src="./demo/index.html?style=./styles/variable-label-placement.json" title="Variable Label Placement" allow="fullscreen" loading="lazy"></iframe>
</div>

Data: OpenFreeMap, OpenStreetMap. GL JS counterpart: [Variable Label Placement](https://maplibre.org/maplibre-gl-js/docs/examples/variable-label-placement/).

## Run it natively

```bash
cargo run -p maplibre-demo -- headed --style docs/src/examples/demo/styles/variable-label-placement.json
```

## Style

<details>
<summary>Style JSON</summary>

```json
{{#include demo/styles/variable-label-placement.json}}
```

</details>
