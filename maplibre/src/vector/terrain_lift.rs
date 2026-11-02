//! Layers that stand on the terrain instead of draping over it: heatmap points take the height
//! of the surface under them, and a building rises from the ground at the middle of its
//! footprint.
use std::{
    hash::{Hash, Hasher},
    sync::Arc,
};

use super::{
    tessellation::{IndexDataType, OverAlignedVertexBuffer},
    AvailableVectorLayerBucket, VectorBufferPool,
};
use crate::{
    coords::WorldTileCoords,
    render::ShaderVertex,
    style::{layer::StyleLayer, Style},
    tcs::{tiles::Tiles, world::World},
    terrain::{coverage::TerrainCoverageIndex, source::dem_source, DemTileComponent},
};

const MAX_VERTICES: usize = 65_536;
pub(crate) type SpatialBuffer = (
    WorldTileCoords,
    String,
    OverAlignedVertexBuffer<ShaderVertex, IndexDataType>,
);

#[derive(Clone, Copy)]
enum Lift {
    /// Stands on the terrain surface, as a heatmap's points do.
    Surface,
    /// Rises from the terrain at the middle of its footprint, as a building does.
    Extrusion,
}

fn lift(layer: &StyleLayer) -> Option<Lift> {
    match layer.type_.as_str() {
        "heatmap" => Some(Lift::Surface),
        "fill-extrusion" => Some(Lift::Extrusion),
        _ => None,
    }
}

#[derive(Default)]
struct CachedBuffers {
    key: u64,
    buffers: Arc<Vec<SpatialBuffer>>,
}

pub(crate) fn for_frame(
    world: &mut World,
    style: &Style,
    sources: &[WorldTileCoords],
) -> (Arc<Vec<SpatialBuffer>>, bool) {
    let key = fingerprint(&world.tiles, style, sources);
    if let Some(cached) = world.resources.get::<CachedBuffers>() {
        if cached.key == key {
            return (cached.buffers.clone(), false);
        }
    }
    let buffers = Arc::new(prepare(&world.tiles, style, sources));
    world.resources.insert(CachedBuffers {
        key,
        buffers: buffers.clone(),
    });
    (buffers, true)
}

fn fingerprint(tiles: &Tiles, style: &Style, sources: &[WorldTileCoords]) -> u64 {
    let mut hash = std::hash::DefaultHasher::new();
    style
        .terrain
        .as_ref()
        .map(|terrain| terrain.exaggeration.to_bits())
        .hash(&mut hash);
    for tile in tiles.tiles.values() {
        if let Some(DemTileComponent::Loaded(dem)) = tiles.query::<&DemTileComponent>(tile.coords) {
            tile.coords.hash(&mut hash);
            dem.revision.hash(&mut hash);
            dem.tile.min().to_bits().hash(&mut hash);
            dem.tile.max().to_bits().hash(&mut hash);
        }
    }
    for coords in sources {
        coords.hash(&mut hash);
        if let Some(pending) = tiles.query::<&super::content::LayerReplacements>(*coords) {
            pending.revision.hash(&mut hash);
        }
        for bucket in super::content::vector_layers(tiles, *coords) {
            bucket.buffer.buffer.vertices.len().hash(&mut hash);
        }
    }
    hash.finish()
}

fn prepare(tiles: &Tiles, style: &Style, sources: &[WorldTileCoords]) -> Vec<SpatialBuffer> {
    let Some(dem) = dem_source(style) else {
        return Vec::new();
    };
    let index = TerrainCoverageIndex::build(sources.iter().copied(), tiles, &dem);
    let mut output = Vec::new();
    let mut remaining = MAX_VERTICES;
    for coords in sources {
        for bucket in super::content::vector_layers(tiles, *coords) {
            let Some(layer) = style.layers.iter().find(|l| l.id == bucket.style_layer_id) else {
                continue;
            };
            let Some(lift) = lift(layer) else {
                continue;
            };
            let count = bucket.buffer.buffer.vertices.len();
            if count == 0 || count > remaining {
                continue;
            }
            let mut buffer = OverAlignedVertexBuffer::from_iters(
                bucket.buffer.buffer.vertices.iter().copied(),
                bucket.buffer.buffer.indices.iter().copied(),
                bucket.buffer.usable_indices,
            );
            elevate(&mut buffer.buffer.vertices, bucket, lift, |position| {
                let scale = 2_f64.powi(i32::from(u8::from(coords.z)));
                index.loaded_ground_cached(
                    tiles,
                    (f64::from(coords.x) + f64::from(position[0]) / 4096.0) / scale,
                    (f64::from(coords.y) + f64::from(position[1]) / 4096.0) / scale,
                    crate::terrain::DemDetail::Finest,
                )
            });
            remaining -= count;
            output.push((*coords, layer.id.clone(), buffer));
        }
    }
    output
}

fn elevate(
    vertices: &mut [ShaderVertex],
    bucket: &AvailableVectorLayerBucket,
    lift: Lift,
    sample: impl Fn([f32; 2]) -> Option<f64>,
) {
    let mut start = 0;
    for count in &bucket.feature_indices {
        let end = (start + *count as usize).min(vertices.len());
        if let Some(feature) = vertices.get_mut(start..end) {
            elevate_span(feature, lift, &sample);
        }
        start = end;
    }
}

fn elevate_span(
    vertices: &mut [ShaderVertex],
    lift: Lift,
    sample: &impl Fn([f32; 2]) -> Option<f64>,
) {
    match lift {
        Lift::Extrusion => {
            // The footprint's middle decides the ground, so a building is not sheared by a slope.
            let count = vertices.len().max(1) as f32;
            let middle = vertices.iter().fold([0.0_f32; 2], |sum, vertex| {
                [sum[0] + vertex.position[0], sum[1] + vertex.position[1]]
            });
            if let Some(ground) = sample([middle[0] / count, middle[1] / count]) {
                for vertex in vertices {
                    vertex.distance += ground as f32;
                    vertex.elevation += ground as f32;
                }
            }
        }
        Lift::Surface => {
            for vertex in vertices {
                if let Some(ground) = sample(vertex.position) {
                    vertex.elevation = ground as f32;
                }
            }
        }
    }
}

pub(crate) fn refresh_gpu(
    pool: &VectorBufferPool,
    queue: &crate::render::upload_queue::UploadQueue,
    buffers: &[SpatialBuffer],
    tiles: &Tiles,
) {
    for (coords, id, buffer) in buffers {
        // Pending geometry must not overwrite an allocation whose indices and paint are retained.
        if tiles
            .query::<&super::content::LayerReplacements>(*coords)
            .is_some_and(|pending| {
                pending
                    .vector
                    .iter()
                    .any(|layer| layer.style_layer_id == *id)
            })
        {
            continue;
        }
        if let Some(entry) = pool
            .index()
            .get_layers(*coords)
            .and_then(|layers| layers.iter().find(|entry| entry.style_layer.id == *id))
        {
            let bytes = bytemuck::cast_slice(&buffer.buffer.vertices);
            let range = entry.vertices_buffer_range();
            if bytes.len() as u64 == range.end - range.start {
                queue.write_buffer(pool.vertices(), range.start, bytes);
            }
        }
    }
}

#[cfg(test)]
mod tests;

#[cfg(all(test, feature = "headless", feature = "thread-safe-futures"))]
mod pixels;
