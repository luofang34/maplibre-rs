# Sky, Fog and Terrain

The `sky` properties colour the sky and blend distant ground into fog past the horizon of a steep view.

<div class="map-frame">
  <iframe src="./demo/index.html?style=./styles/sky-fog-terrain.json" title="Sky, Fog and Terrain" allow="fullscreen" loading="lazy"></iframe>
</div>

Data: Esri World Street Map, MapLibre terrain tiles. GL JS counterpart: [Sky, Fog and Terrain](https://maplibre.org/maplibre-gl-js/docs/examples/sky-fog-terrain/).

## Run it natively

```bash
cargo run -p maplibre-demo -- headed --style docs/src/examples/demo/styles/sky-fog-terrain.json --max-pitch 85
```

## Style

<details>
<summary>Style JSON</summary>

```json
{{#include demo/styles/sky-fog-terrain.json}}
```

</details>
