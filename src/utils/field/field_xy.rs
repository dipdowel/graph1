use crate::core::context::GraphContext;
use crate::primitives::numeric::Numeric;
use crate::primitives::plane::RectArea;
use crate::primitives::point::Point;
use std::cell::RefCell;
use std::ops::Deref;

type InfluenceFn<T, UserData, PointState: Clone> = fn(
    &mut GraphContext<UserData>,
    &FieldXYInfluencer<T, UserData, PointState>,
    &mut FieldXYPoint<T, PointState>,
    &FieldXY<T, PointState>,
);

//// Represents a point that influences a field of points in 2D space.
pub struct FieldXYInfluencer<'a, T: Numeric, UserData, PointState: Clone> {
    location: Point<T>,
    /** Any additional data that can be used to influence a point */
    // influence_data: Option<InfluenceData>,
    magnitude: Point<T>,
    // field: &'a FieldXY<T>
    field: Option<&'a FieldXY<T, PointState>>,

    pub function: InfluenceFn<T, UserData, PointState>,
}

impl<'a, T: Numeric, UserData, PointState: Clone> FieldXYInfluencer<'a, T, UserData, PointState> {
    pub fn new(
        location: Point<T>,
        magnitude: Point<T>,
        function: InfluenceFn<T, UserData, PointState>,
    ) -> Self {
        FieldXYInfluencer {
            location,
            magnitude,
            field: None,
            function,
        }
    }

    pub fn set_field(&mut self, field: &'a FieldXY<T, PointState>) {
        self.field = Some(field);
        // Ensure the location is within the bounds of the field rectangle
        // self.location.x = self.location.x.clamp(field_rect.top_left.x, field_rect.bottom_right().x);
        // self.location.y = self.location.y.clamp(field_rect.top_left.y, field_rect.bottom_right().y);
    }

    pub fn get_location(&self) -> &Point<T> {
        &self.location
    }

    pub fn set_location(&mut self, location: Point<T>) {
        self.location = location;
    }
    pub fn get_magnitude(&self) -> &Point<T> {
        &self.magnitude
    }
    // pub fn get_influence_data(&self) -> Option<&InfluenceData> {
    //     if &self.influence_data {
    //        return Some(&self.influence_data);
    //     }
    //         None
    //
    // }
}

/// Represents a strategy for applying influence from an influencer to a field point.
// pub trait InfluenceStrategy<T: Numeric> {
//     fn apply(
//         &mut self,
//         target: &mut FieldXYPoint<T>,
//         influencer: &FieldXYInfluencer<T>,
//         // field: &FieldXY<T>,
//     );
// }

/// Represents a point in a 2D field that can be influenced by an influencer.
#[derive(Debug, Copy, Clone)]
pub struct FieldXYPoint<T: Numeric, PointState: Clone> {
    /** 1-d index of the field point */
    index: usize,
    /** 2-d index of the field point */
    index_2d: Point<usize>,
    /** "on-screen" position of the point */
    location: Point<T>,
    /** How sensitive the point is to an influencer */
    sensitivity: Point<T>,

    /** State of each field point, can be used to store additional information */
    state: PointState,
}

impl<T: Numeric, PointState: Clone> FieldXYPoint<T, PointState> {
    pub fn new(
        location: Point<T>,
        index: usize,
        index_2d: Point<usize>,
        sensitivity: Point<T>,
        state: PointState,
    ) -> Self {
        FieldXYPoint {
            index,
            index_2d,
            location,
            sensitivity,
            state,
        }
    }

    pub fn get_location(&self) -> &Point<T> {
        &self.location
    }
    pub fn set_location(&mut self, location: Point<T>) {
        self.location = location;
    }

    pub fn get_state(&self) -> &PointState {
        &self.state
    }

    pub fn set_state(&mut self, state: PointState) {
        self.state = state;
    }


}

// impl<T: Numeric> FieldXYPoint<T> {
//     pub fn apply_influence<Strategy: InfluenceStrategy<T>>(
//         &mut self,
//         strategy: &mut Strategy,
//         influencer: &FieldXYInfluencer<T>,
//         field: &FieldXY<T>
//     ) {
//         // strategy.apply(self, &influencer,  &field);
//         strategy.apply(self, &influencer);
//     }
// }

#[derive(Debug, Clone)]
pub struct FieldXY<T: Numeric, PointState: Clone> {
    /** Points of the field */
    points: RefCell<Vec<FieldXYPoint<T, PointState>>>,
    // field_matrix: RefCell<Vec<Vec<FieldXYPoint<T>>>>,
    // points: Vec<FieldXYPoint<T>>,
    field_rect: RectArea<T>,
}

impl<T: Numeric, PointState: Clone> FieldXY<T, PointState> {
    pub fn new(
        field_rect: RectArea<T>,
        point_template: FieldXYPoint<T, PointState>,
        num_points: usize,
    ) -> Self {
        let cols = (num_points as f64).sqrt().ceil() as usize;
        let rows = (num_points + cols - 1) / cols;
        // let mut points: Vec<FieldXYPoint<T>> = Vec::with_capacity(num_points);
        let step_x = field_rect.dimensions.w.to_u32() / cols as u32;
        let step_y = field_rect.dimensions.h.to_u32() / rows as u32;

        // let field_matrix: RefCell<Vec<Vec<FieldXYPoint<T>>>> = RefCell::new(vec![vec![]; rows]);
        let points: RefCell<Vec<FieldXYPoint<T, PointState>>> =
            RefCell::new(Vec::with_capacity(num_points));
        let mut field_matrix: Vec<Vec<&FieldXYPoint<T, PointState>>> = vec![vec![]; rows];
        // let mut points: Vec<FieldXYPoint<T>> = Vec::with_capacity(num_points);

        for row in 0..rows {
            for col in 0..cols {
                let index = row * cols + col;
                if index >= num_points {
                    break; // Avoid adding more points than requested
                }
                let x = field_rect.top_left.x + T::from_u32(col as u32 * step_x);
                let y = field_rect.top_left.y + T::from_u32(row as u32 * step_y);
                let location = Point::new(x, y);
                let index_2d = Point::new(col, row);
                let point = FieldXYPoint::new(
                    location,
                    index,
                    index_2d,
                    point_template.sensitivity.clone(),
                    point_template.state.clone(),
                );
                // field_matrix[row][col] = &point;

                points.borrow_mut().push(point);
            }
        }

        FieldXY { points, field_rect }
    }
    pub fn get_points(&self) -> &RefCell<Vec<FieldXYPoint<T, PointState>>> {
        &self.points
    }

    // pub fn get_field_matrix(&self) -> &Vec<Vec<FieldXYPoint<T>>> {
    //     &self.field_matrix
    // }
    // pub fn get_field_rect(&self) -> &RectArea<T> {
    //     &self.field_rect
    // }
    // pub fn get_point(&self, index: usize) -> Option<&FieldXYPoint<T>> {
    //     let point = self.points.get(index);
    //     if let Some(p) = point {
    //         return Some(&p);
    //     }
    //     None
    // }
    // pub fn get_point_2d(&self, index_2d: &Point<usize>) -> Option<FieldXYPoint<T>> {
    //     if let Some(row) = self.field_matrix.get(index_2d.y) {
    //         if let Some(point) = row.get(index_2d.x) {
    //             return Some(*point);
    //         }
    //     }
    //     None
    // }
    // pub fn get_point_by_location(&self, location: &Point<T>) -> Option<FieldXYPoint<T>> {
    //     self.points
    //         .iter()
    //         .find(|p| p.location == *location)
    //         .cloned()
    // }
    //
    // pub fn get_point_by_approx_location(
    //     &self,
    //     location: &Point<T>,
    //     max_offset: Option<Point<T>>,
    // ) -> Option<&FieldXYPoint<T>> {
    //     let max_offset = max_offset.unwrap_or(Point::new(T::zero(), T::zero()));
    //
    //     let min_x = location.x - max_offset.x;
    //     let max_x = location.x + max_offset.x;
    //     let min_y = location.y - max_offset.y;
    //     let max_y = location.y + max_offset.y;
    //
    //     self.points.iter().find(|p| {
    //         p.location.x >= min_x
    //             && p.location.x <= max_x
    //             && p.location.y >= min_y
    //             && p.location.y <= max_y
    //     })
    // }

    pub fn apply_influencer_fn<UserData>(
        &self,
        ctx: &mut GraphContext<UserData>,
        influencer: &FieldXYInfluencer<T, UserData, PointState>,
    ) {
        for point in self.points.borrow_mut().iter_mut() {
            (influencer.function)(ctx, influencer, point, self);
        }
    }
}
