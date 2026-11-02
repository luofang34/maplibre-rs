# Elevate Symbols above the Terrain

Labels follow the terrain surface and stand above it by `symbol-height-offset` metres.

<div class="map-frame">
  <iframe src="./demo/index.html?style=./styles/elevate-symbols-above-the-terrain.json" title="Elevate Symbols above the Terrain" allow="fullscreen" loading="lazy"></iframe>
</div>

Data: Esri World Street Map, MapLibre terrain tiles. GL JS counterpart: [Elevate Symbols above the Terrain](https://maplibre.org/maplibre-gl-js/docs/examples/elevate-symbols-above-the-terrain/).

## Run it natively

```bash
cargo run -p maplibre-demo -- headed --style docs/src/examples/demo/styles/elevate-symbols-above-the-terrain.json --max-pitch 85
```

## Style

<details>
<summary>Style JSON</summary>

```json
{{#include demo/styles/elevate-symbols-above-the-terrain.json}}
```

</details>
