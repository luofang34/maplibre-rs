# User Guide

maplibre-rs renders [MapLibre style documents](https://maplibre.org/maplibre-style-spec/) with WebGPU on desktop, mobile and the web.

## Desktop

Open any style JSON in an interactive window:

```bash
cargo run -p maplibre-demo -- headed --style docs/src/examples/demo/styles/display-a-map.json
```

`--globe` and `--terrain` open the bundled globe and terrain styles, and `--projection` overrides the style's projection. Left-drag pans, the wheel zooms, right-drag rotates and tilts.

## Web

`startMapLibre` takes the paths of the wasm and worker files, which default to the bundled ones, and a style document as a string:

```js
import { startMapLibre } from 'maplibre-rs'

const style = await (await fetch('https://demotiles.maplibre.org/style.json')).json()
await startMapLibre(undefined, undefined, JSON.stringify(style))
```

The page needs a `<canvas id="maplibre">`. [Building Libraries](../development-guide/building-libraries.md) explains the WebGL and multithreaded builds. Adding `?spector` to the page address of a WebGL build opens the Spector frame debugger.

## From here

[Style Support](./style-support.md) lists what a style can use. The [Examples](../examples/index.md) show each feature running, and the [Runtime API](../examples/runtime-api.md) changes a running map from Rust.
