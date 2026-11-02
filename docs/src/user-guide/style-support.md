# Style Support

maplibre-rs reads the [MapLibre Style Spec](https://maplibre.org/maplibre-style-spec/).

| Part of a style | Supported |
|-----------------|-----------|
| Layers | `background`, `fill`, `line`, `symbol`, `circle`, `heatmap`, `fill-extrusion`, `raster`, `hillshade`, `color-relief` |
| Sources | `vector`, `raster`, `raster-dem`, `geojson` (with clustering), `image` |
| Tile formats | Mapbox Vector Tiles, MapLibre Tiles, PNG, JPEG, WebP, PMTiles archives through `pmtiles://` |
| Projections | `mercator`, `globe`, `vertical-perspective`, and arrays and expressions that blend them |
| Scene | `terrain`, `sky`, `light`, root camera properties, global `state` |

Not supported yet: `video` and `canvas` sources, `fill-extrusion-rounded-corner-distance` and CSS `font-faces`. A style that names an unknown source type or projection fails to load.

A `raster-dem` source given by `url` takes its `encoding` from the style, so a terrarium TileJSON needs `"encoding": "terrarium"` next to the `url`.
