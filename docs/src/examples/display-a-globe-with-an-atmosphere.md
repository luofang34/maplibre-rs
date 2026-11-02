# Display a Globe with an Atmosphere

A satellite globe with the style `sky` atmosphere around the limb, lit by the root `light` position.

<div class="map-frame">
  <iframe src="./demo/index.html?style=./styles/display-a-globe-with-an-atmosphere.json" title="Display a Globe with an Atmosphere" allow="fullscreen" loading="lazy"></iframe>
</div>

Data: Sentinel-2 cloudless by EOX. GL JS counterpart: [Display a Globe with an Atmosphere](https://maplibre.org/maplibre-gl-js/docs/examples/display-a-globe-with-an-atmosphere/).

## Run it natively

```bash
cargo run -p maplibre-demo -- headed --style docs/src/examples/demo/styles/display-a-globe-with-an-atmosphere.json
```

## Style

<details>
<summary>Style JSON</summary>

```json
{{#include demo/styles/display-a-globe-with-an-atmosphere.json}}
```

</details>
