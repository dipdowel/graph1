/// Represents different types of neighborhoods in a uniform 2D grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NeighborhoodType {

    /// Includes all 8 surrounding cells: the 4 orthogonal and 4 diagonal neighbors.
    ///
    /// Also known as the **Moore neighborhood**.
    ///
    /// ### Example:
    ///```text
    /// * * *
    /// * x *
    /// * * *
    ///```
    /// ### Use Cases:
    /// - Image filtering (3x3 kernels)
    /// - Game of Life
    /// - Local spatial interaction
    Immediate,
    
    /// Includes the 4 orthogonal (non-diagonal) neighbors:
    /// - Top (0, -1)
    /// - Right (1, 0)
    /// - Bottom (0, 1)
    /// - Left (-1, 0)
    ///
    /// Also known as the **Von Neumann neighborhood**.
    ///
    /// ### Example:
    ///```text
    /// . * .
    /// * x *
    /// . * .
    ///```
    ///
    /// ### Use Cases:
    /// - Grid-based pathfinding (non-diagonal movement)
    /// - Cellular automata with orthogonal interaction
    Orthogonal,

    /// Includes only the 4 diagonal neighbors:
    /// - Top-Left, Top-Right, Bottom-Left, Bottom-Right
    ///
    /// ### Example:
    ///```text
    /// * . *
    /// . x .
    /// * . *
    ///```
    ///
    /// ### Use Cases:
    /// - Diagonal-only pathfinding
    /// - Symmetry and diagonal-based simulations
    Diagonal,

    /// Includes all cells within a square area of side length `(2 * radius + 1)`.
    /// Grows outward in axis-aligned steps.
    ///
    /// ### Use Cases:
    /// - Local field interaction
    /// - Gaussian filters or blurs
    Square {
        /// Number of cells to cover, outwards from the given center
        distance: usize,
    },

    /// Includes all cells within a circular radius (based on Euclidean distance).
    ///
    /// ### Use Cases:
    /// - Radial effects like lighting or explosion radius
    /// - Smooth influence propagation
    Circle {
        /// Maximum Euclidean distance from center
        radius: usize,
    },

    /// Includes all cells within a given Manhattan distance.
    /// Forms a diamond-shaped neighborhood.
    ///
    /// ### Use Cases:
    /// - Grid distance maps
    /// - Fire spread, infection simulations
    Diamond {
        /// Maximum Manhattan distance (|dx| + |dy|)
        distance: usize,
    },

/*
    // TODO: this type looks promising. See how it can be improved and implemented
    /// Includes only cells in a specified direction and within a certain angular width.
    ///
    /// ### Use Cases:
    /// - Vision cones for AI
    /// - Directed particle spread
    DirectionalSector {
        /// Direction angle in degrees or radians (0 = right, 90 = up, etc.)
        angle: f32,
        /// Width of the sector in degrees or radians (e.g. 90 for quarter circle)
        aperture: f32,
        /// Maximum radius from center
        radius: usize,
    },
    
 */
}