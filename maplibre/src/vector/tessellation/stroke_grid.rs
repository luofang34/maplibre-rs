//! Strokes split along the globe's grid of tile cells, so a line follows the curve of the
//! sphere.
use geozero::error::{GeozeroError, Result as GeoResult};
use lyon::geom;

use super::{MaxIndex, ZeroTessellator};
use crate::{
    projection::globe::subdivision::subdivide_line_segment,
    style::{layer::StyleLayer, line_stroke::LineStroke},
};

impl<I> ZeroTessellator<I>
where
    I: std::ops::Add + From<lyon::tessellation::VertexId> + MaxIndex + Copy + Into<u32>,
{
    /// Strokes features as `layer` lays them out: with its cap and join.
    pub fn set_layer_strokes(&mut self, layer: &StyleLayer) {
        self.stroke = LineStroke::of_layer(layer);
        self.join_property = LineStroke::join_property(layer);
    }

    pub(super) fn append_coordinate(&mut self, coordinate: [f32; 2]) -> GeoResult<()> {
        if let Some(previous) = self.contour_last {
            let points = subdivide_line_segment(previous, coordinate, self.subdivision_granularity)
                .map_err(|error| GeozeroError::Geometry(error.to_string()))?;
            for point in points {
                self.path_builder
                    .borrow_mut()
                    .line_to(geom::point(point[0], point[1]));
            }
        } else {
            self.path_builder
                .borrow_mut()
                .begin(geom::point(coordinate[0], coordinate[1]));
            self.contour_start = Some(coordinate);
            self.path_open = true;
        }
        self.contour_last = Some(coordinate);
        Ok(())
    }

    pub(super) fn subdivide_closing_segment(&mut self) -> GeoResult<()> {
        let (Some(start), Some(last)) = (self.contour_start, self.contour_last) else {
            return Ok(());
        };
        let mut points = subdivide_line_segment(last, start, self.subdivision_granularity)
            .map_err(|error| GeozeroError::Geometry(error.to_string()))?;
        points.pop();
        for point in points {
            self.path_builder
                .borrow_mut()
                .line_to(geom::point(point[0], point[1]));
        }
        Ok(())
    }
}
