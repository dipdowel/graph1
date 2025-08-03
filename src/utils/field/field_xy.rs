use crate::primitives::numeric::Numeric;
use crate::primitives::plane::{Dimensions2d, RectArea};
use crate::primitives::point::Point;
use std::cell::RefCell;

/// Function type that defines how an influencer affects a point in the field.
type InfluenceFn<T, PointState: Clone> = fn(
    win_dimensions: Dimensions2d,
    &FieldXYInfluencer<T, PointState>,
    &mut FieldXYPoint<T, PointState>,
    &FieldXY<T, PointState>,
);

//// An influencer is a point that influences a field of points in 2D space.
#[derive(Debug, Clone)]
pub struct FieldXYInfluencer<'a, T: Numeric, PointState: Clone> {

    /// The location of the influencer in the field, represented as a Point.
    location: Point<T>,

    /// The magnitude of the influence along the x and y axes (represented as a Point).
    /// This can be used to define the strength and direction of the influence.
    magnitude: Point<T>,

    /// Optional reference to the field that this influencer is associated with.
    field: Option<&'a FieldXY<'a, T, PointState>>,

    /// The function that defines how the influencer affects the points in the field.
    pub function: InfluenceFn<T, PointState>,
}


impl<'a, T: Numeric, PointState: Clone> FieldXYInfluencer<'a, T, PointState> {
    pub fn new(
        location: Point<T>,
        magnitude: Point<T>,
        function: InfluenceFn<T, PointState>,
    ) -> Self {
        FieldXYInfluencer {
            location,
            magnitude,
            field: None,
            function,
        }
    }

    pub fn default() -> Self {
        FieldXYInfluencer {
            location: Point::new(T::zero(), T::zero()),
            magnitude: Point::new(T::zero(), T::zero()),
            field: None,
            function: |_, _, _, _| panic!(" no influencer function provided! "),
        }
    }

    /// Sets the field that this influencer is associated with.
    pub fn set_field(&mut self, field: &'a FieldXY<T, PointState>) {
        self.field = Some(field);
    }

    /// Returns the current location of the influencer.
    pub fn get_location(&self) -> &Point<T> {
        &self.location
    }

    /// Sets the location of the influencer.
    pub fn set_location(&mut self, location: Point<T>) {
        self.location = location;
    }

    /// Returns the current magnitude of the influence.
    pub fn get_magnitude(&self) -> &Point<T> {
        &self.magnitude
    }

    /// Sets the magnitude of the influence.
    pub fn set_magnitude(&mut self, magnitude: Point<T>) {
        self.magnitude = magnitude;
    }
}


/// Represents a point in a 2D field that can be influenced by an influencer (e.g., a mouse cursor).
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
    /** State of each field point, can be used to store any extra information about the point */
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

    /// Returns the coordinates of the point in the field.
    pub fn get_location(&self) -> &Point<T> {
        &self.location
    }

    /// Sets the coordinates of the point in the field.
    pub fn set_location(&mut self, location: Point<T>) {
        self.location = location;
    }

    /// Returns the current state of the point (
    pub fn get_state(&self) -> &PointState {
        &self.state
    }

    /// Sets the state of the point.
    pub fn set_state(&mut self, state: PointState) {
        self.state = state;
    }
}


/// Represents a 2D field of points that can be influenced by an influencer.
#[derive(Debug, Clone)]
pub struct FieldXY<'a, T: Numeric, PointState: Clone> {
    /// Points of the field
    points: RefCell<Vec<FieldXYPoint<T, PointState>>>,
    /// The rectangular area that defines the field's boundaries.
    field_rect: RectArea<T>,
    /// The influencer that affects the points in the field.
    influencer: FieldXYInfluencer<'a, T, PointState>,
    /// Dimensions of the window of the application
    win_dimensions: Dimensions2d,
}

impl<'a, T: Numeric, PointState: Clone> FieldXY<'a, T, PointState> {
    pub fn new(
        field_rect: RectArea<T>,
        point_template: FieldXYPoint<T, PointState>,
        num_points: usize,
        win_dimensions: Dimensions2d,
        influencer: FieldXYInfluencer<'a, T, PointState>,
    ) -> Self {
        let cols = (num_points as f64).sqrt().ceil() as usize;
        let rows = (num_points + cols - 1) / cols;

        let step_x = field_rect.dimensions.w.to_u32() / cols as u32;
        let step_y = field_rect.dimensions.h.to_u32() / rows as u32;


        let points: RefCell<Vec<FieldXYPoint<T, PointState>>> =
            RefCell::new(Vec::with_capacity(num_points));
        // let mut field_matrix: Vec<Vec<&FieldXYPoint<T, PointState>>> = vec![vec![]; rows];


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

        FieldXY {
            points,
            field_rect,
            influencer,
            win_dimensions,
        }
    }

    pub fn default() -> Self {
        FieldXY {
            points: RefCell::new(Vec::new()),
            field_rect: RectArea {
                top_left: Point::new(T::zero(), T::zero()),
                dimensions: Dimensions2d {
                    w: T::zero(),
                    h: T::zero(),
                },
                color: None,
            },
            influencer: FieldXYInfluencer::default(),
            win_dimensions: Dimensions2d { w: 0, h: 0 },
        }
    }

    /// Returns a reference to the points in the field.
    pub fn get_points(&self) -> &RefCell<Vec<FieldXYPoint<T, PointState>>> {
        &self.points
    }


    ///
    pub fn get_field_rect(&self) -> &RectArea<T> {
        &self.field_rect
    }

    /// Sets the influencer function and updates the window dimensions.
    pub fn set_influencer_fn(
        &mut self,
        win_dimensions: Dimensions2d,
        influencer: FieldXYInfluencer<'a, T, PointState>,
    ) {
        self.influencer = influencer;
        self.win_dimensions = win_dimensions;
    }

    pub fn update_win_dimensions(&mut self, win_dimensions: Dimensions2d) {
        self.win_dimensions = win_dimensions;
    }

    /// Returns a mutable reference to the influencer.
    pub fn borrow_influencer_mut(&mut self) -> &mut FieldXYInfluencer<'a, T, PointState> {
        &mut self.influencer
    }

    /// Applies the influence of the influencer to points in the field.
    /// If the `win_dimensions` is zero, it returns an error.
    /// Otherwise, it iterates over the points and applies the influence function to each point.
    pub fn influence(&mut self) -> Result<(), String> {
        if self.win_dimensions.is_zero() {
            return Err(String::from(
                "win_dimensions is zero, cannot apply influence",
            ));
        }
        for point in self.points.borrow_mut().iter_mut() {
            (self.influencer.function)(self.win_dimensions, &self.influencer, point, self);
        }
        Ok(())
    }
    // pub fn get_point(&self, index: usize) -> Option<&FieldXYPoint<T, PointState>> {
    //     self.points.borrow().get(index)
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
}
