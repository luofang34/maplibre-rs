import { startMapLibre } from 'maplibre-rs'

// `?style=<url>` loads any style; `?projection=globe|mercator|vertical-perspective` overrides its
// projection, and `?lat=`, `?lon=`, `?zoom=`, `?pitch=`, `?bearing=` override its initial camera.
// Without a style URL the MapLibre demotiles world style is shown on the globe.
const params = new URLSearchParams(location.search)
const customStyleUrl = params.get('style')
const styleUrl = customStyleUrl ?? 'https://demotiles.maplibre.org/style.json'
const projection = params.get('projection') ?? (customStyleUrl ? null : 'globe')

const numberParam = (name: string): number | null => {
  const raw = params.get(name)
  if (raw === null || raw.trim() === '') return null
  const parsed = Number(raw)
  return Number.isFinite(parsed) ? parsed : null
}

const loadStyle = async (): Promise<string | undefined> => {
  try {
    const response = await fetch(styleUrl)
    if (!response.ok) {
      throw new Error(`${response.status} ${response.statusText}`)
    }
    const style = await response.json()
    if (projection) {
      style.projection = { type: projection }
    }
    const lat = numberParam('lat')
    const lon = numberParam('lon')
    if (lat !== null || lon !== null) {
      const base = Array.isArray(style.center) ? style.center : [0, 0]
      style.center = [lon ?? base[0], lat ?? base[1]]
    }
    const zoom = numberParam('zoom')
    if (zoom !== null) style.zoom = zoom
    const pitch = numberParam('pitch')
    if (pitch !== null) style.pitch = pitch
    const bearing = numberParam('bearing')
    if (bearing !== null) style.bearing = bearing
    return JSON.stringify(style)
  } catch (error) {
    console.error(`Cannot load style ${styleUrl}, falling back to the built-in style`, error)
    return undefined
  }
}

loadStyle().then((styleJson) => startMapLibre(undefined, undefined, styleJson))
