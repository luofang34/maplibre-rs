# PMTiles Source

A `pmtiles://` URL reads tiles out of one archive with HTTP range requests, with no tile server.

<div class="map-frame">
  <iframe src="./demo/index.html?style=./styles/pmtiles-source-and-protocol.json" title="PMTiles Source" allow="fullscreen" loading="lazy"></iframe>
</div>

Data: Protomaps, OpenStreetMap. GL JS counterpart: [PMTiles Source](https://maplibre.org/maplibre-gl-js/docs/examples/pmtiles-source-and-protocol/).

## Run it natively

```bash
cargo run -p maplibre-demo -- headed --style docs/src/examples/demo/styles/pmtiles-source-and-protocol.json
```

## Style

<details>
<summary>Style JSON</summary>

```json
{{#include demo/styles/pmtiles-source-and-protocol.json}}
```

</details>
