use crate::primitives::neighborhood::NeighborhoodType;
use crate::primitives::plane::Dimensions2d;
use crate::primitives::point::Point;
use std::fmt::Debug;

/// Defines how to handle cells at the edge of the grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoundaryPolicy {
    /// Clamp coordinates to grid boundaries (edge cells repeat).
    Clamp,
    /// Wrap coordinates around (toroidal topology).
    Wrap,
    /// Mirror coordinates at boundaries (reflect).
    Mirror,
}

/// Represents a cell in the discrete rule field with multi-channel state.
#[derive(Debug, Clone)]
pub struct Cell<CellState: Clone> {
    /// The state of the cell (can store color, brightness, flags, or other properties).
    pub state: CellState,
}

impl<CellState: Clone> Cell<CellState> {
    pub fn new(state: CellState) -> Self {
        Cell { state }
    }
}

/// A function type that defines a rule for updating a cell based on its neighborhood.
/// Parameters:
/// - grid: The current (immutable) grid
/// - cell_coords: Coordinates of the cell being updated
/// - neighborhood_type: The type of neighborhood to inspect
/// - boundary_policy: How to handle edge cells
/// Returns: The new state for the cell
pub type RuleFn<CellState> = fn(
    grid: &DiscreteRuleField<CellState>,
    cell_coords: Point<usize>,
    neighborhood_type: NeighborhoodType,
    boundary_policy: BoundaryPolicy,
) -> CellState;

/// A rule set that can be applied to cells in the field.
#[derive(Clone)]
pub struct RuleSet<CellState: Clone> {
    /// The neighborhood type to inspect when applying this rule.
    pub neighborhood_type: NeighborhoodType,
    /// The rule function that computes the next state.
    pub rule_fn: RuleFn<CellState>,
}

impl<CellState: Clone> RuleSet<CellState> {
    pub fn new(neighborhood_type: NeighborhoodType, rule_fn: RuleFn<CellState>) -> Self {
        RuleSet {
            neighborhood_type,
            rule_fn,
        }
    }
}

/// Configuration for how the field should be updated.
pub struct UpdateConfig {
    /// Neighborhood type used by the update function.
    pub update_neighborhood: NeighborhoodType,
    /// Center point of the update neighborhood.
    pub update_neighborhood_center: Point<usize>,
    /// If true, the whole field gets updated and `update_neighborhood` and `update_neighborhood_center` are ignored
    pub update_whole_field: bool,
}

impl UpdateConfig {
    pub fn new(
        update_neighborhood: NeighborhoodType,
        update_neighborhood_center: Point<usize>,
        update_whole_field: bool,
    ) -> Self {
        UpdateConfig {
            update_neighborhood,
            update_neighborhood_center,
            update_whole_field,
        }
    }
}

/// A 2D discrete rule field that implements cellular automaton / rule-based stencil computation.
pub struct DiscreteRuleField<CellState: Clone> {
    /// Current state grid (row-major order).
    grid: Vec<Cell<CellState>>,
    /// Next state grid (used for double buffering).
    next_grid: Vec<Cell<CellState>>,
    /// Dimensions of the grid.
    dimensions: Dimensions2d<usize>,
    /// Boundary policy for edge cells.
    boundary_policy: BoundaryPolicy,
    /// Default rule set applied to all cells (unless overridden).
    default_rule: RuleSet<CellState>,
    /// Storage for override rule sets (sparse).
    override_rules: Vec<RuleSet<CellState>>,
    /// Index mapping for each cell: None = use default_rule, Some(idx) = use override_rules[idx].
    /// Has 1:1 correspondence with grid (same length).
    rule_indices: Vec<Option<usize>>,
    /// details on how exactly the field needs to be updated
    pub update_config: UpdateConfig

}

impl<CellState: Clone> DiscreteRuleField<CellState> {
    /// Creates a new discrete rule field with uniform rule assignment.
    /// All cells will initially use the provided default rule.
    /// Individual cells can be assigned custom rules using `set_cell_rule()`.
    pub fn new(
        dimensions: Dimensions2d<usize>,
        initial_state: CellState,
        boundary_policy: BoundaryPolicy,
        rule_set: RuleSet<CellState>,
        update_neighborhood: NeighborhoodType,
    ) -> Self {
        let capacity = dimensions.w * dimensions.h;
        let grid = vec![Cell::new(initial_state.clone()); capacity];
        let next_grid = vec![Cell::new(initial_state); capacity];
        let rule_indices = vec![None; capacity]; // All cells use default rule

        DiscreteRuleField {
            grid,
            next_grid,
            dimensions,
            boundary_policy,
            default_rule: rule_set,
            override_rules: Vec::new(),
            rule_indices,
            update_config: UpdateConfig::new(
                update_neighborhood,
                Point::new(dimensions.w / 2, dimensions.h / 2),
                false,
            )
        }
    }

    /// Returns the dimensions of the field.
    pub fn dimensions(&self) -> &Dimensions2d<usize> {
        &self.dimensions
    }

    /// Returns the boundary policy.
    pub fn boundary_policy(&self) -> BoundaryPolicy {
        self.boundary_policy
    }

    /// Sets the update configuration for the field.
    /// This allows you to control which cells get updated and the update neighborhood.
    ///
    /// # Example
    /// ```ignore
    /// let new_config = UpdateConfig::new(
    ///     NeighborhoodType::Circle { radius: 10 },
    ///     Point::new(50, 50),
    ///     false,
    /// );
    /// field.set_update_config(new_config);
    /// ```
    pub fn set_update_config(&mut self, config: UpdateConfig) {
        self.update_config = config;
    }

    /// Sets a custom rule for a specific cell. The cell will use this rule instead of the default rule.
    ///
    /// # Arguments
    /// * `coords` - The coordinates of the cell
    /// * `rule_set` - The rule set to apply to this cell
    ///
    /// # Returns
    /// * `Ok(())` if the rule was successfully set
    /// * `Err(String)` if the coordinates are invalid
    ///
    /// # Example
    /// ```ignore
    /// let custom_rule = RuleSet::new(NeighborhoodType::Orthogonal, my_rule_fn);
    /// field.set_cell_rule(Point::new(5, 5), custom_rule)?;
    /// ```
    pub fn set_cell_rule(&mut self, coords: Point<usize>, rule_set: RuleSet<CellState>) -> Result<(), String> {
        let adjusted = self
            .apply_boundary_policy(coords)
            .ok_or("Coordinates out of bounds")?;
        let index = self.coords_to_index(adjusted);

        if index >= self.rule_indices.len() {
            return Err("Invalid cell index".to_string());
        }

        // Check if this cell already has an override
        if let Some(override_idx) = self.rule_indices[index] {
            // Replace existing override
            self.override_rules[override_idx] = rule_set;
        } else {
            // Add new override
            self.override_rules.push(rule_set);
            self.rule_indices[index] = Some(self.override_rules.len() - 1);
        }

        Ok(())
    }

    /// Resets a cell's rule to the default rule (removes any override).
    ///
    /// # Arguments
    /// * `coords` - The coordinates of the cell
    ///
    /// # Returns
    /// * `Ok(())` if the rule was successfully reset
    /// * `Err(String)` if the coordinates are invalid
    ///
    /// # Example
    /// ```ignore
    /// field.reset_cell_rule(Point::new(5, 5))?;
    /// ```
    pub fn reset_cell_rule(&mut self, coords: Point<usize>) -> Result<(), String> {
        let adjusted = self
            .apply_boundary_policy(coords)
            .ok_or("Coordinates out of bounds")?;
        let index = self.coords_to_index(adjusted);

        if index >= self.rule_indices.len() {
            return Err("Invalid cell index".to_string());
        }

        // Simply set to None to use default rule
        // Note: We don't remove from override_rules to avoid index invalidation
        self.rule_indices[index] = None;

        Ok(())
    }

    /// Gets the rule set for a specific cell (either override or default).
    ///
    /// # Arguments
    /// * `coords` - The coordinates of the cell
    ///
    /// # Returns
    /// A reference to the rule set that applies to this cell
    ///
    /// # Example
    /// ```ignore
    /// let rule = field.get_cell_rule(Point::new(5, 5));
    /// ```
    pub fn get_cell_rule(&self, coords: Point<usize>) -> &RuleSet<CellState> {
        let index = self.coords_to_index(coords);
        match self.rule_indices.get(index) {
            Some(Some(override_idx)) => &self.override_rules[*override_idx],
            _ => &self.default_rule,
        }
    }

    /// Gets a cell at the given coordinates (with boundary policy applied).
    pub fn get_cell(&self, coords: Point<usize>) -> Option<&Cell<CellState>> {
        let adjusted = self.apply_boundary_policy(coords)?;
        let index = self.coords_to_index(adjusted);
        self.grid.get(index)
    }

    /// Gets a mutable reference to a cell at the given coordinates (with boundary policy applied).
    /// This allows direct modification of the cell state.
    ///
    /// # Example
    /// ```ignore
    /// if let Some(cell) = field.get_cell_mut(Point::new(5, 5)) {
    ///     cell.state = new_state;
    /// }
    /// ```
    pub fn get_cell_mut(&mut self, coords: Point<usize>) -> Option<&mut Cell<CellState>> {
        let adjusted = self.apply_boundary_policy(coords)?;
        let index = self.coords_to_index(adjusted);
        self.grid.get_mut(index)
    }

    /// Gets a cell at the given coordinates without boundary checks (unsafe but fast).
    pub fn get_cell_unchecked(&self, coords: Point<usize>) -> &Cell<CellState> {
        let index = self.coords_to_index(coords);
        &self.grid[index]
    }

    /// Gets a mutable cell at the given coordinates without boundary checks (unsafe but fast).
    ///
    /// # Safety
    /// The caller must ensure that coordinates are within bounds.
    /// Out of bounds access will panic.
    ///
    /// # Example
    /// ```ignore
    /// let coords = Point::new(5, 5);
    /// if coords.x < field.dimensions().w && coords.y < field.dimensions().h {
    ///     let cell = field.get_cell_unchecked_mut(coords);
    ///     cell.state = new_state;
    /// }
    /// ```
    pub fn get_cell_unchecked_mut(&mut self, coords: Point<usize>) -> &mut Cell<CellState> {
        let index = self.coords_to_index(coords);
        &mut self.grid[index]
    }

    /// Sets a cell state at the given coordinates.
    ///
    /// # Arguments
    /// * `coords` - The coordinates of the cell to modify
    /// * `state` - The new state to set
    ///
    /// # Returns
    /// * `Ok(())` if the cell was successfully updated
    /// * `Err(String)` if the coordinates are invalid
    ///
    /// # Example
    /// ```ignore
    /// field.set_cell(Point::new(5, 5), new_state)?;
    /// ```
    pub fn set_cell(&mut self, coords: Point<usize>, state: CellState) -> Result<(), String> {
        let adjusted = self
            .apply_boundary_policy(coords)
            .ok_or("Coordinates out of bounds")?;
        let index = self.coords_to_index(adjusted);
        if let Some(cell) = self.grid.get_mut(index) {
            cell.state = state;
            Ok(())
        } else {
            Err("Invalid cell index".to_string())
        }
    }

    /// Gets the neighbor coordinates based on the neighborhood type and center coordinates.
    pub fn get_neighbors(
        &self,
        center: Point<usize>,
        neighborhood: NeighborhoodType,
    ) -> Vec<Point<usize>> {
        let mut neighbors = Vec::new();

        match neighborhood {
            NeighborhoodType::Immediate => {
                // Moore neighborhood (8 neighbors)
                let offsets = [
                    (-1, -1),
                    (0, -1),
                    (1, -1),
                    (-1, 0),
                    (1, 0),
                    (-1, 1),
                    (0, 1),
                    (1, 1),
                ];
                for (dx, dy) in offsets.iter() {
                    if let Some(coord) = self.apply_offset(center, *dx, *dy) {
                        neighbors.push(coord);
                    }
                }
            }
            NeighborhoodType::Orthogonal => {
                // Von Neumann neighborhood (4 neighbors)
                let offsets = [(0, -1), (1, 0), (0, 1), (-1, 0)];
                for (dx, dy) in offsets.iter() {
                    if let Some(coord) = self.apply_offset(center, *dx, *dy) {
                        neighbors.push(coord);
                    }
                }
            }
            NeighborhoodType::Diagonal => {
                // 4 diagonal neighbors
                let offsets = [(-1, -1), (1, -1), (-1, 1), (1, 1)];
                for (dx, dy) in offsets.iter() {
                    if let Some(coord) = self.apply_offset(center, *dx, *dy) {
                        neighbors.push(coord);
                    }
                }
            }
            NeighborhoodType::Square { distance } => {
                let dist = distance as i32;
                for dy in -dist..=dist {
                    for dx in -dist..=dist {
                        if dx == 0 && dy == 0 {
                            continue; // Skip center
                        }
                        if let Some(coord) = self.apply_offset(center, dx, dy) {
                            neighbors.push(coord);
                        }
                    }
                }
            }
            NeighborhoodType::Circle { radius } => {
                let r_sq = (radius * radius) as i32;
                let r = radius as i32;
                for dy in -r..=r {
                    for dx in -r..=r {
                        if dx == 0 && dy == 0 {
                            continue; // Skip center
                        }
                        let dist_sq = dx * dx + dy * dy;
                        if dist_sq <= r_sq {
                            if let Some(coord) = self.apply_offset(center, dx, dy) {
                                neighbors.push(coord);
                            }
                        }
                    }
                }
            }
            NeighborhoodType::Diamond { distance } => {
                let dist = distance as i32;
                for dy in -dist..=dist {
                    for dx in -dist..=dist {
                        if dx == 0 && dy == 0 {
                            continue; // Skip center
                        }
                        let manhattan = dx.abs() + dy.abs();
                        if manhattan <= dist {
                            if let Some(coord) = self.apply_offset(center, dx, dy) {
                                neighbors.push(coord);
                            }
                        }
                    }
                }
            }
        }

        neighbors
    }

    /// Updates the field by one time step using the assigned rules.
    /// This reads from the current grid and writes to the next grid, then swaps them.
    pub fn update(&mut self, one_time_config:Option<UpdateConfig>) {


        let mut config = &self.update_config;

        if one_time_config.is_some(){
            config = one_time_config.as_ref().unwrap();
        }


        // Process all cells based on the update neighborhood
        let cells_to_process = self.get_cells_to_process(config);

        for cell_coords in cells_to_process {
            let rule_set = self.get_rule_set_for_cell(cell_coords);
            let new_state = (rule_set.rule_fn)(
                self,
                cell_coords,
                rule_set.neighborhood_type,
                self.boundary_policy,
            );

            let index = self.coords_to_index(cell_coords);
            self.next_grid[index].state = new_state;
        }

        // Swap buffers
        std::mem::swap(&mut self.grid, &mut self.next_grid);
    }

    /// Gets all cells that should be processed based on the update neighborhood.
    fn get_cells_to_process(&self, config:&UpdateConfig) -> Vec<Point<usize>> {
        if config.update_whole_field {
            let mut cells = Vec::with_capacity(self.dimensions.w * self.dimensions.h);
            for y in 0..self.dimensions.h {
                for x in 0..self.dimensions.w {
                    cells.push(Point::new(x, y));
                }
            }
            return cells;
        } else {
            // Get cells in the update neighborhood around the center point
            let mut cells = self.get_neighbors(
                config.update_neighborhood_center,
                config.update_neighborhood,
            );

            // Include the center cell itself
            cells.push(config.update_neighborhood_center);

            cells
        }

    }

    /// Gets the rule set for a specific cell.
    fn get_rule_set_for_cell(&self, coords: Point<usize>) -> &RuleSet<CellState> {
        let index = self.coords_to_index(coords);
        match self.rule_indices.get(index) {
            Some(Some(override_idx)) => &self.override_rules[*override_idx],
            _ => &self.default_rule,
        }
    }

    /// Converts 2D coordinates to a 1D index (row-major order).
    fn coords_to_index(&self, coords: Point<usize>) -> usize {
        coords.y * self.dimensions.w + coords.x
    }

    /// Converts a 1D index to 2D coordinates.
    pub fn index_to_coords(&self, index: usize) -> Point<usize> {
        Point::new(index % self.dimensions.w, index / self.dimensions.w)
    }

    /// Applies boundary policy to coordinates.
    fn apply_boundary_policy(&self, coords: Point<usize>) -> Option<Point<usize>> {
        match self.boundary_policy {
            BoundaryPolicy::Clamp => Some(Point::new(
                coords.x.min(self.dimensions.w - 1),
                coords.y.min(self.dimensions.h - 1),
            )),
            BoundaryPolicy::Wrap => Some(Point::new(
                coords.x % self.dimensions.w,
                coords.y % self.dimensions.h,
            )),
            BoundaryPolicy::Mirror => {
                let x = if coords.x >= self.dimensions.w {
                    self.dimensions.w - 1 - (coords.x % self.dimensions.w)
                } else {
                    coords.x
                };
                let y = if coords.y >= self.dimensions.h {
                    self.dimensions.h - 1 - (coords.y % self.dimensions.h)
                } else {
                    coords.y
                };
                Some(Point::new(x, y))
            }
        }
    }

    /// Applies an offset to coordinates with boundary policy handling.
    fn apply_offset(&self, coords: Point<usize>, dx: i32, dy: i32) -> Option<Point<usize>> {
        let new_x = coords.x as i32 + dx;
        let new_y = coords.y as i32 + dy;

        match self.boundary_policy {
            BoundaryPolicy::Clamp => {
                if new_x < 0 || new_y < 0 {
                    return Some(Point::new(new_x.max(0) as usize, new_y.max(0) as usize));
                }
                Some(Point::new(
                    (new_x as usize).min(self.dimensions.w - 1),
                    (new_y as usize).min(self.dimensions.h - 1),
                ))
            }
            BoundaryPolicy::Wrap => {
                let wrapped_x = ((new_x % self.dimensions.w as i32) + self.dimensions.w as i32)
                    % self.dimensions.w as i32;
                let wrapped_y = ((new_y % self.dimensions.h as i32) + self.dimensions.h as i32)
                    % self.dimensions.h as i32;
                Some(Point::new(wrapped_x as usize, wrapped_y as usize))
            }
            BoundaryPolicy::Mirror => {
                if new_x < 0
                    || new_x >= self.dimensions.w as i32
                    || new_y < 0
                    || new_y >= self.dimensions.h as i32
                {
                    let mirrored_x = if new_x < 0 {
                        (-new_x) as usize
                    } else if new_x >= self.dimensions.w as i32 {
                        self.dimensions.w - 1 - ((new_x - self.dimensions.w as i32) as usize)
                    } else {
                        new_x as usize
                    };
                    let mirrored_y = if new_y < 0 {
                        (-new_y) as usize
                    } else if new_y >= self.dimensions.h as i32 {
                        self.dimensions.h - 1 - ((new_y - self.dimensions.h as i32) as usize)
                    } else {
                        new_y as usize
                    };
                    Some(Point::new(
                        mirrored_x.min(self.dimensions.w - 1),
                        mirrored_y.min(self.dimensions.h - 1),
                    ))
                } else {
                    Some(Point::new(new_x as usize, new_y as usize))
                }
            }
        }
    }

    /// Returns a reference to the current grid.
    pub fn grid(&self) -> &Vec<Cell<CellState>> {
        &self.grid
    }

    /// Returns a mutable reference to the current grid.
    pub fn grid_mut(&mut self) -> &mut Vec<Cell<CellState>> {
        &mut self.grid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct SimpleState {
        value: u8,
    }

    fn simple_rule(
        grid: &DiscreteRuleField<SimpleState>,
        cell_coords: Point<usize>,
        neighborhood_type: NeighborhoodType,
        _boundary_policy: BoundaryPolicy,
    ) -> SimpleState {
        let neighbors = grid.get_neighbors(cell_coords, neighborhood_type);
        let sum: u32 = neighbors
            .iter()
            .filter_map(|&coord| grid.get_cell(coord))
            .map(|cell| cell.state.value as u32)
            .sum();
        let avg = if neighbors.is_empty() {
            0
        } else {
            (sum / neighbors.len() as u32) as u8
        };
        SimpleState { value: avg }
    }

    #[test]
    fn test_field_creation() {
        let dimensions = Dimensions2d::new(10, 10);
        let initial_state = SimpleState { value: 0 };
        let rule_set = RuleSet::new(NeighborhoodType::Immediate, simple_rule);

        let field = DiscreteRuleField::<SimpleState>::new(
            dimensions,
            initial_state,
            BoundaryPolicy::Clamp,
            rule_set,
            NeighborhoodType::Immediate,
        );

        assert_eq!(field.dimensions().w, 10);
        assert_eq!(field.dimensions().h, 10);
        assert_eq!(field.grid().len(), 100);
    }

    #[test]
    fn test_get_neighbors_immediate() {
        let dimensions = Dimensions2d::new(5, 5);
        let initial_state = SimpleState { value: 0 };
        let rule_set = RuleSet::new(NeighborhoodType::Immediate, simple_rule);

        let field = DiscreteRuleField::<SimpleState>::new(
            dimensions,
            initial_state,
            BoundaryPolicy::Clamp,
            rule_set,
            NeighborhoodType::Immediate,
        );

        let center = Point::new(2, 2);
        let neighbors = field.get_neighbors(center, NeighborhoodType::Immediate);

        assert_eq!(neighbors.len(), 8); // Moore neighborhood has 8 neighbors
    }

    #[test]
    fn test_get_neighbors_orthogonal() {
        let dimensions = Dimensions2d::new(5, 5);
        let initial_state = SimpleState { value: 0 };
        let rule_set = RuleSet::new(NeighborhoodType::Orthogonal, simple_rule);

        let field = DiscreteRuleField::<SimpleState>::new(
            dimensions,
            initial_state,
            BoundaryPolicy::Clamp,
            rule_set,
            NeighborhoodType::Orthogonal,
        );

        let center = Point::new(2, 2);
        let neighbors = field.get_neighbors(center, NeighborhoodType::Orthogonal);

        assert_eq!(neighbors.len(), 4); // Von Neumann neighborhood has 4 neighbors
    }

    #[test]
    fn test_update() {
        let dimensions = Dimensions2d::new(3, 3);
        let initial_state = SimpleState { value: 1 };
        let rule_set = RuleSet::new(NeighborhoodType::Immediate, simple_rule);

        let mut field = DiscreteRuleField::<SimpleState>::new(
            dimensions,
            initial_state,
            BoundaryPolicy::Clamp,
            rule_set,
            NeighborhoodType::Immediate,
        );

        // Set center cell to a different value
        field
            .set_cell(Point::new(1, 1), SimpleState { value: 8 })
            .unwrap();

        // Update should average neighbors
        field.update(Some(UpdateConfig::new(
            NeighborhoodType::Immediate,
            Point::new(1, 1),
            true,
        )));

        // After update, cells should have new values based on their neighbors
        let center_cell = field.get_cell(Point::new(1, 1)).unwrap();
        assert!(center_cell.state.value <= 8);
    }

    #[test]
    fn test_set_cell_rule() {
        let dimensions = Dimensions2d::new(5, 5);
        let initial_state = SimpleState { value: 0 };
        let default_rule = RuleSet::new(NeighborhoodType::Immediate, simple_rule);

        let mut field = DiscreteRuleField::<SimpleState>::new(
            dimensions,
            initial_state,
            BoundaryPolicy::Clamp,
            default_rule,
            NeighborhoodType::Immediate,
        );

        // Define a custom rule that always returns value 42
        fn custom_rule(
            _grid: &DiscreteRuleField<SimpleState>,
            _cell_coords: Point<usize>,
            _neighborhood_type: NeighborhoodType,
            _boundary_policy: BoundaryPolicy,
        ) -> SimpleState {
            SimpleState { value: 42 }
        }

        let custom_rule_set = RuleSet::new(NeighborhoodType::Orthogonal, custom_rule);

        // Set custom rule for specific cell
        let custom_cell = Point::new(2, 2);
        field.set_cell_rule(custom_cell, custom_rule_set).unwrap();

        // Verify the cell uses the custom rule
        let rule = field.get_cell_rule(custom_cell);
        assert_eq!(rule.neighborhood_type, NeighborhoodType::Orthogonal);

        // Update the field
        field.update(Some(UpdateConfig::new(
            NeighborhoodType::Immediate,
            Point::new(2, 2),
            true,
        )));

        // The cell with custom rule should have value 42
        let cell_with_custom_rule = field.get_cell(custom_cell).unwrap();
        assert_eq!(cell_with_custom_rule.state.value, 42);

        // Other cells should use the default rule (averaging)
        let other_cell = field.get_cell(Point::new(0, 0)).unwrap();
        assert_eq!(other_cell.state.value, 0); // No neighbors with different values

        // Reset the cell rule
        field.reset_cell_rule(custom_cell).unwrap();

        // Verify it now uses the default rule
        let rule_after_reset = field.get_cell_rule(custom_cell);
        assert_eq!(rule_after_reset.neighborhood_type, NeighborhoodType::Immediate);
    }
}

