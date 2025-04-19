use crate::primitives::numeric::Numeric;
use crate::primitives::plane::RectArea;
use crate::utils::math::geometry::region::Region;

#[derive(Clone, Copy, Debug)]
pub struct Quadrants<T: Numeric = u32> {
    pub top_left: Region<T>,
    pub top_right: Region<T>,
    pub bottom_left: Region<T>,
    pub bottom_right: Region<T>,
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

    /// Returns a vector of references to the four quadrants
    /// In the following order: top-left, top-right, bottom-right,bottom-left
    pub fn get_quadrants(&self) -> Vec<&Region<u32>> {
        vec![
            &self.top_left,
            &self.top_right,
            &self.bottom_right,
            &self.bottom_left,
        ]
    }
}


impl<T: Numeric> Quadrants<T> {

    /// Converts all `Region<T>` fields to another numeric type, returning a new `Quadrants<U>`.
    /// Useful for rendering or calculation scenarios involving a different numeric precision.
    pub fn convert<U: Numeric>(&self) -> Quadrants<U> {
        Quadrants {
            top_left: self.top_left.convert::<U>(),
            top_right: self.top_right.convert::<U>(),
            bottom_left: self.bottom_left.convert::<U>(),
            bottom_right: self.bottom_right.convert::<U>(),
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

    #[test]
    fn get_quadrants_should_return_correct_order() {
        let quads = Quadrants::from_dimensions(100, 100);
        let regions = quads.get_quadrants();

        assert_eq!(regions.len(), 4);
        assert!(std::ptr::eq(regions[0], &quads.top_left));
        assert!(std::ptr::eq(regions[1], &quads.top_right));
        assert!(std::ptr::eq(regions[2], &quads.bottom_right));
        assert!(std::ptr::eq(regions[3], &quads.bottom_left));
    }
    #[test]
    fn quadrants_convert_should_preserve_layout() {
        let quads_u32 = Quadrants::from_dimensions(100, 100);
        let quads_i32 = quads_u32.convert::<i32>();

        assert_eq!(quads_u32.top_left.area() as i32, quads_i32.top_left.area());
        assert_eq!(quads_u32.top_right.center().x as i32, quads_i32.top_right.center().x);
        assert_eq!(quads_u32.bottom_right.size().w as i32, quads_i32.bottom_right.size().w);
    }


}
