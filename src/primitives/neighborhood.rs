/// Represents different types of neighborhoods in a uniform 2D grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NeighborhoodType {
    /// The Von Neumann neighborhood includes the 4 orthogonal (non-diagonal) neighbors:
    /// - Top (0, -1)
    /// - Right (1, 0)
    /// - Bottom (0, 1)
    /// - Left (-1, 0)
    ///
    /// ### Example:
    /// 
    /// . # .
    /// # x #
    /// . # .
    /// 
    ///
    /// ### Use Cases:
    /// - Cellular automata (e.g., Conway’s Game of Life variant)
    /// - Manhattan-style pathfinding (no diagonal movement)
    VonNeumann,

    /// The Moore neighborhood includes all 8 surrounding cells:
    /// - Orthogonal and Diagonal neighbors
    ///
    /// ### Example:
    /// 
    /// # # #
    /// # x #
    /// # # #
    /// 
    ///
    /// ### Use Cases:
    /// - Image filtering (e.g., 3x3 convolution kernels)
    /// - Cellular automata (standard Game of Life)
    Moore,

    /// Includes only the 4 diagonal neighbors:
    /// - Top-Left (-1, -1)
    /// - Top-Right (1, -1)
    /// - Bottom-Left (-1, 1)
    /// - Bottom-Right (1, 1)
    ///
    /// ### Example:
    /// 
    /// # . #
    /// . x .
    /// # . #
    /// 
    ///
    /// ### Use Cases:
    /// - Symmetry operations
    /// - Diagonal flow or interaction checks
    DiagonalOnly,

    /// Includes all cells within a square of side length `(2 * radius + 1)`.
    /// Can generalize Moore and Von Neumann for larger distances.
    ///
    /// ### Use Cases:
    /// - Local area simulations
    /// - Gaussian blur and other extended kernels
    ExtendedSquare {
        /// Radius around the center cell (number of steps outwards)
        radius: usize,
    },

    /// Includes all cells within a circular area defined by Euclidean distance.
    ///
    /// ### Use Cases:
    /// - Smooth falloff filters
    /// - Light propagation and radial field effects
    Circular {
        /// Maximum Euclidean distance from the center
        radius: usize,
    },

    /// Includes all cells within a Manhattan distance ≤ `radius`.
    /// Forms a diamond shape in the grid.
    ///
    /// ### Use Cases:
    /// - Movement and reachability maps in grid-based games
    /// - Heatmap simulations
    Manhattan {
        /// Maximum Manhattan distance (|dx| + |dy|)
        radius: usize,
    },

    /// Includes all cells within a Chebyshev distance ≤ `radius`.
    /// Equivalent to the maximum of |dx| and |dy| ≤ radius.
    ///
    /// ### Use Cases:
    /// - Simulates square-area influence
    /// - Generalized Moore neighborhood
    Chebyshev {
        /// Maximum Chebyshev distance
        radius: usize,
    },
/*
    /// Includes neighbors only in a specific direction or angular sector.
    /// Used for directional effects.
    ///
    /// ### Use Cases:
    /// - Vision cones in games
    /// - Field-of-view for AI agents
    Directional {
        /// Angle (degrees or radians) of direction from the center (e.g. 0 = right)
        angle: usize,
        /// Angular width of the sector (e.g. 90 degrees for a quarter circle)
        aperture: usize,
        /// Maximum radius to include
        radius: usize,
    },
    */
}
