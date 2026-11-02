# Display Buildings in 3D

`fill-extrusion` lifts building footprints to the height in their vector tile attributes.

<div class="map-frame">
  <iframe src="./demo/index.html?style=./styles/display-buildings-in-3d.json" title="Display Buildings in 3D" allow="fullscreen" loading="lazy"></iframe>
</div>

Data: OpenFreeMap, OpenStreetMap. GL JS counterpart: [Display Buildings in 3D](https://maplibre.org/maplibre-gl-js/docs/examples/display-buildings-in-3d/).

## Run it natively

```bash
cargo run -p maplibre-demo -- headed --style docs/src/examples/demo/styles/display-buildings-in-3d.json
```

## Style

<details>
<summary>Style JSON</summary>

```json
{{#include demo/styles/display-buildings-in-3d.json}}
```

</details>
