//! Labels a host reads from and keeps off the offscreen map: the symbols placement accepted,
//! and the screen areas the host draws over itself.

use super::HeadlessMap;

impl HeadlessMap {
    /// Symbols accepted by placement under a point or box, narrowed by layers and a filter.
    pub fn query_rendered_symbols_in(
        &self,
        geometry: crate::sdf::query::QueryGeometry,
        options: &crate::sdf::query::QueryOptions,
    ) -> Result<Vec<crate::sdf::query::RenderedSymbol>, crate::sdf::query::QueryError> {
        crate::sdf::query::query_rendered_symbols_in(
            &self.map_context.world,
            &self.map_context.style,
            geometry,
            options,
        )
    }

    /// Symbols accepted by placement at a screen point, optionally restricted to style layers.
    pub fn query_rendered_symbols(
        &self,
        point: [f64; 2],
        layers: Option<&[&str]>,
    ) -> Vec<crate::sdf::query::RenderedSymbol> {
        crate::sdf::query::query_rendered_symbols(
            &self.map_context.world,
            &self.map_context.style,
            point,
            layers,
        )
    }
}
