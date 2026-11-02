# Display a Line that Crosses the 180th Meridian

Continuous coordinates past 180° keep a Pacific line in one piece, and `line-center` puts its label midway.

<div class="map-frame">
  <iframe src="./demo/index.html?style=./styles/display-line-that-crosses-180th-meridian.json" title="Display a Line that Crosses the 180th Meridian" allow="fullscreen" loading="lazy"></iframe>
</div>

Data: MapLibre demotiles. GL JS counterpart: [Display a Line that Crosses the 180th Meridian](https://maplibre.org/maplibre-gl-js/docs/examples/display-line-that-crosses-180th-meridian/).

## Run it natively

```bash
cargo run -p maplibre-demo -- headed --style docs/src/examples/demo/styles/display-line-that-crosses-180th-meridian.json
```

## Style

<details>
<summary>Style JSON</summary>

```json
{{#include demo/styles/display-line-that-crosses-180th-meridian.json}}
```

</details>
