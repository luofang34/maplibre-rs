//! The tile covering a frame selects, and the history the next frame selects against.

use crate::{
    coords::{ViewRegion, WorldTileCoords, ZoomLevel},
    projection::lod_history::LodHistory,
    raster::RasterSourceId,
    render::{
        projection::{raster_source_regions, view_region_for_projection, ProjectionStateError},
        view_state::{ViewState, ViewStatePadding},
    },
    style::Style,
    tcs::world::World,
};

/// Tiles per raster source, as [`raster_source_regions`] selects them.
pub type RasterCoverings = Vec<(RasterSourceId, Vec<WorldTileCoords>)>;

/// The tiles the previous frame drew, per covering, as one immutable snapshot every request
/// and draw of this frame selects against. Each covering keeps its own history: the terrain
/// surface, the view and each raster source split the quadtree at different levels, so one
/// covering's leaves say nothing about where another split.
#[derive(Clone, Default)]
pub(crate) struct FrameLodHistory {
    pub(crate) view: LodHistory,
    pub(crate) raster: std::collections::HashMap<RasterSourceId, LodHistory>,
    pub(crate) terrain: LodHistory,
}

/// Takes the snapshot for the frame about to select tiles: a camera that moves a little each
/// frame would otherwise switch a tile sitting on a level threshold between its fine and coarse
/// version frame after frame.
pub(crate) fn snapshot_lod_history(world: &mut World) {
    let shared = world.resources.get::<SharedCovering>();
    let history = FrameLodHistory {
        view: LodHistory::new(
            shared
                .and_then(|shared| shared.tiles.as_ref())
                .map_or(&[], |(tiles, _)| tiles.as_slice()),
        ),
        raster: shared
            .map(|shared| {
                shared
                    .raster
                    .iter()
                    .map(|(name, tiles)| (name.clone(), LodHistory::new(tiles)))
                    .collect()
            })
            .unwrap_or_default(),
        terrain: LodHistory::new(crate::terrain::drawn_surface_tiles(world)),
    };
    world.resources.insert(history);
}

/// The drawn tiles a frame selected.
#[derive(Clone, Debug)]
pub struct SharedCovering {
    tiles: Option<(Vec<WorldTileCoords>, ZoomLevel)>,
    raster: RasterCoverings,
}

impl SharedCovering {
    /// The tiles selected, or `None` when no region was.
    pub(crate) fn tiles(&self) -> Option<&[WorldTileCoords]> {
        self.tiles.as_ref().map(|(tiles, _)| tiles.as_slice())
    }
}

/// The tiles the frame's covering selected, each once, or `None` before a covering is selected
/// or when it selected no region.
pub(crate) fn drawn_tiles(world: &World) -> Option<&[WorldTileCoords]> {
    world.resources.get::<SharedCovering>()?.tiles()
}

/// The tiles the frame draws at `level`, with the raster tiles each raster source draws, kept
/// as the next frame's level history.
pub(crate) fn drawn_covering(
    style: &Style,
    view_state: &ViewState,
    world: &mut World,
    level: ZoomLevel,
) -> Result<(Option<ViewRegion>, RasterCoverings), ProjectionStateError> {
    let region =
        view_region_for_projection(style, view_state, world, level, ViewStatePadding::Tight)?;
    let raster = raster_source_regions(style, view_state, world, ViewStatePadding::Tight)?;
    world.resources.insert(SharedCovering {
        tiles: region
            .as_ref()
            .map(|region| (region.iter().collect(), region.zoom_level())),
        raster: raster.clone(),
    });
    Ok((region, raster))
}

#[cfg(test)]
mod tests;
