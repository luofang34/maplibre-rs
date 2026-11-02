//! Uploads data to the GPU which is needed for rendering.
use std::collections::BTreeSet;

use crate::{
    context::MapContext,
    coords::WorldTileCoords,
    raster::{
        dem_border::{filled_sides, with_border},
        resource::RasterResources,
        AvailableRasterLayerData, RasterLayerData, RasterLayersDataComponent,
    },
    render::{
        eventually::{Eventually, Eventually::Initialized},
        memory_budget::{FrameUploads, UploadKind},
        Renderer,
    },
    tcs::{
        system::{SystemError, SystemResult},
        tiles::Tiles,
    },
};

pub fn upload_system(
    MapContext {
        world,
        renderer: Renderer { device, queue, .. },
        ..
    }: &mut MapContext,
) -> SystemResult {
    world.resources.get_or_init_mut::<FrameUploads>();
    let Some((Initialized(raster_resources), budget)) = world
        .resources
        .query_mut::<(&mut Eventually<RasterResources>, &mut FrameUploads)>()
    else {
        return Err(SystemError::Dependencies);
    };

    // Every loaded tile gets its texture, as GL JS uploads a raster tile when it arrives, as
    // far as the frame's image floor reaches; the retention system bounds the set by evicting
    // tiles that left the view.
    let source_tiles: BTreeSet<WorldTileCoords> =
        world.tiles.tiles.values().map(|tile| tile.coords).collect();
    let deferred = upload_raster_layer(
        raster_resources,
        (device, queue),
        &world.tiles,
        source_tiles,
        budget,
    );
    if deferred {
        crate::render::frame_signals::keep_animating(world);
    }

    Ok(())
}

/// Uploads the textures of loaded tiles that the frame's budget admits; `true` when some wait
/// for a later frame.
#[tracing::instrument(skip_all)]
fn upload_raster_layer(
    raster_resources: &mut RasterResources,
    (device, queue): (&wgpu::Device, &crate::render::upload_queue::UploadQueue),
    tiles: &Tiles,
    source_tiles: BTreeSet<WorldTileCoords>,
    budget: &mut FrameUploads,
) -> bool {
    let mut mipmapped = Vec::new();
    let mut deferred = false;
    'tiles: for coords in source_tiles {
        let Some(layers) = tiles.query::<&RasterLayersDataComponent>(coords) else {
            continue;
        };
        for data in &layers.layers {
            if let RasterLayerData::Available(data) = data {
                // A result that lands after its source left the style is never drawn.
                if raster_resources.draws(&data.source)
                    && raster_resources
                        .get_bound_texture(&data.source, &coords)
                        .is_none()
                {
                    if !budget.take(UploadKind::Image, data.image.as_raw().len()) {
                        deferred = true;
                        break 'tiles;
                    }
                    if let Some(texture) =
                        upload_image(raster_resources, device, queue, tiles, data)
                    {
                        mipmapped.push(texture);
                    }
                }
            }
        }
    }
    // One submission fills every new texture's mip chain.
    raster_resources.generate_mipmaps(device, queue, &mipmapped);
    deferred
}

/// The image of `source` loaded at each tile around `coords`, indexed `[dy + 1][dx + 1]`.
fn neighbour_images<'a>(
    tiles: &'a Tiles,
    source: &crate::raster::RasterSourceId,
    coords: WorldTileCoords,
) -> [[Option<&'a image::RgbaImage>; 3]; 3] {
    let mut images: [[Option<&image::RgbaImage>; 3]; 3] = Default::default();
    for (neighbour, (dx, dy)) in crate::terrain::backfill::neighbours(coords) {
        images[(dy + 1) as usize][(dx + 1) as usize] = tiles
            .query::<&RasterLayersDataComponent>(neighbour)
            .and_then(|component| {
                component.layers.iter().find_map(|layer| match layer {
                    RasterLayerData::Available(data) if &data.source == source => Some(&data.image),
                    _ => None,
                })
            });
    }
    images
}

/// Uploads and binds the tile's texture; returns the texture when its mip chain is still to be
/// filled.
fn upload_image(
    raster_resources: &mut RasterResources,
    device: &wgpu::Device,
    queue: &crate::render::upload_queue::UploadQueue,
    tiles: &Tiles,
    data: &AvailableRasterLayerData,
) -> Option<wgpu::Texture> {
    let bordered;
    let mut border_sides = None;
    let image = if raster_resources.has_border(&data.source) {
        let neighbours = neighbour_images(tiles, &data.source, data.coords);
        border_sides = Some(filled_sides(&data.image, &neighbours));
        bordered = with_border(&data.image, &neighbours);
        &bordered
    } else {
        &data.image
    };
    let (width, height) = image.dimensions();

    // Raster style colors are sampled in the encoded color space, matching WebGL's default
    // RGBA upload path. An sRGB texture view would decode the texels to linear values before
    // writing them to the non-sRGB render target, making imagery too dark.
    let mipmapped = border_sides.is_none();
    let texture = if mipmapped {
        raster_resources.create_imagery_texture(device, width, height)
    } else {
        raster_resources.create_texture(
            None,
            device,
            wgpu::TextureFormat::Rgba8Unorm,
            width,
            height,
            wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        )
    };

    queue.write_texture(
        wgpu::TexelCopyTextureInfo {
            aspect: wgpu::TextureAspect::All,
            texture: &texture.texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
        },
        image,
        wgpu::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(4 * width),
            rows_per_image: Some(height),
        },
        texture.size,
    );

    let mips = mipmapped.then(|| texture.texture.clone());
    raster_resources.bind_texture(device, &data.source, &data.coords, texture);
    if let Some(sides) = border_sides {
        raster_resources.set_border_sides(&data.source, data.coords, sides);
    }
    mips
}
