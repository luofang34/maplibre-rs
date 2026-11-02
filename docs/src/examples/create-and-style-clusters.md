# Create and Style Clusters

GeoJSON `cluster` sources group points in the worker; `step` expressions style each cluster by size.

<div class="map-frame">
  <iframe src="./demo/index.html?style=./styles/create-and-style-clusters.json" title="Create and Style Clusters" allow="fullscreen" loading="lazy"></iframe>
</div>

Data: MapLibre demotiles, USGS earthquakes. GL JS counterpart: [Create and Style Clusters](https://maplibre.org/maplibre-gl-js/docs/examples/create-and-style-clusters/).

## Run it natively

```bash
cargo run -p maplibre-demo -- headed --style docs/src/examples/demo/styles/create-and-style-clusters.json
```

## Style

<details>
<summary>Style JSON</summary>

```json
{{#include demo/styles/create-and-style-clusters.json}}
```

</details>
