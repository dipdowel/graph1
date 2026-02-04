# Cellular automaton / rule-based stencil computation (execution on CPU)

## Overview

The system operates on a 2D grid of cells, where each cell stores a multi-channel state (e.g. color, brightness, flags, other properties).
Time advances in discrete steps via an update function. At each step, the next state of every cell in a local neighborhood is computed. The local neighborhood is specified by a value of type `NeighborhoodType` from `src/primitives/neighborhood.rs` and a provided central cell, so the update function must accept those two parameters: the `NeighborhoodType`  and the coordinates of the central cell.

Execution proceeds in synchronous passes. For each iteration, the algorithm reads from an immutable input grid and writes results into a separate output grid, then swaps the buffers. In-place updates are avoided to prevent order-dependent behavior.

## Rule assignment

Each cell's update is governed by a rule or a set of rules (further in the document "rule set"). The same rule set can be assigned to all cells, or different rule sets can be assigned to different cells, so the system must support flexible rule set assignment.

For each processed cell, the CPU evaluates a rule set that:

1. Inspects the states of neighboring cells, defined by a value of type `NeighborhoodType`. NB: this is a different neighborhood than the one passed to the update function!
2. Applies conditional logic (thresholds, classifications, priorities, dominance rules) to determine which neighbors influence the currently processed cell in what way.
3. Combines neighbor influences using linear or nonlinear operations (selection, attenuation, exclusion).
4. Produces a new state for the currently processed cell.

## Edge cell handling
Edge cells are handled using a predefined boundary policy (e.g. clamp, mirror, wrap).

## Other considerations
The computation is data-parallel conceptually but executed via cache-friendly nested loops (typically scanline order). The system may be iterated multiple times, producing emergent spatial patterns and temporal dynamics.
 