use crate::primitives::plane::RectArea;
use crate::utils::math::geometry::region::Region;

#[derive(Debug)]
pub struct Quadrants {
    pub top_left: Region<u32>,
    pub top_right: Region<u32>,
    pub bottom_left: Region<u32>,
    pub bottom_right: Region<u32>,
}

impl Quadrants {
    pub fn from_dimensions(w: u32, h: u32) -> Self {
        let full_region = Region::new(RectArea::new(0, 0, w, h, None));
        let (top, bottom) = full_region.split_horizontal(None);
        let (top_left, top_right) = top.split_vertical(None);
        let (bottom_left, bottom_right) = bottom.split_vertical(None);

        Self {
            top_left,
            top_right,
            bottom_left,
            bottom_right,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quadrant_edges_should_align() {
        let w = 800;
        let h = 600;
        let quads = Quadrants::from_dimensions(w, h);

        // Midpoint vertical alignment (top-bottom)
        assert_eq!(quads.top_left.bottom(), quads.bottom_left.top());
        assert_eq!(quads.top_right.bottom(), quads.bottom_right.top());

        // Midpoint horizontal alignment (left-right)
        assert_eq!(quads.top_left.right(), quads.top_right.left());
        assert_eq!(quads.bottom_left.right(), quads.bottom_right.left());
    }

    #[test]
    fn quadrant_sizes_should_match() {
        let w = 1024;
        let h = 768;
        let quads = Quadrants::from_dimensions(w, h);

        let sz = quads.top_left.size();
        assert_eq!(sz, quads.top_right.size());
        assert_eq!(sz, quads.bottom_left.size());
        assert_eq!(sz, quads.bottom_right.size());
    }

    #[test]
    fn quadrants_cover_entire_window_area() {
        let w = 640;
        let h = 480;
        let quads = Quadrants::from_dimensions(w, h);

        let area_sum = quads.top_left.area()
            + quads.top_right.area()
            + quads.bottom_left.area()
            + quads.bottom_right.area();

        assert_eq!(area_sum, w * h);
    }

    #[test]
    fn quadrants_should_cover_area_with_odd_dimensions() {
        let w = 5;
        let h = 3;
        let quads = Quadrants::from_dimensions(w, h);

        let total_area = quads.top_left.area()
            + quads.top_right.area()
            + quads.bottom_left.area()
            + quads.bottom_right.area();

        assert_eq!(total_area, w * h);
    }
}
