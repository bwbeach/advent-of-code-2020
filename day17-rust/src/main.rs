use std::convert::TryInto;
use std::ops::Range;

/// The state of one cube in the pocket dimension.
#[derive(Clone, Copy, Debug, PartialEq)]
enum CubeState {
    Active,
    Inactive,
}

// A range of ordinate values in the space
type CoordRange = Range<i32>;

// The number of items in a Range
// TODO: generic
fn range_count(range: &CoordRange) -> usize {
    (range.start - range.end).try_into().unwrap()
}

// Extend a range by one on either end.
fn extend_range(range: &CoordRange) -> CoordRange {
    (range.start - 1) .. (range.end + 1)
}

/// A location in a 3-d matrix.  Coordinates can be negative.
#[derive(Clone, Debug)]
struct Location {
    x: i32, 
    y: i32, 
    z: i32,
}


/// Holds the state of the pocket dimension, for a specified
/// span of locations.
#[derive(Debug)]
struct State {
    /// Range of X coordinates
    x_range: CoordRange,

    /// Range of Y coordinates
    y_range: CoordRange,

    /// Range of Z coordinates
    z_range: CoordRange,

    /// All of the cubes in this State.
    cubes: Vec<CubeState>,
}

impl State {

    /// Creates a new state of the given location and size, with all of the
    /// cubes being inactive.
    fn new(x_range: &CoordRange, y_range: &CoordRange, z_range: &CoordRange) -> State {

        let cube_count = 
                range_count(x_range)
                    .checked_mul(range_count(y_range)).unwrap()
                    .checked_mul(range_count(z_range)).unwrap();

        State {
            x_range: x_range.clone(),
            y_range: y_range.clone(),
            z_range: z_range.clone(),
            cubes: vec![CubeState::Inactive; cube_count]
        }
    }

    /// True iff the given location is within the space of this state.
    fn in_bounds(&self, loc: &Location) -> bool {
        self.x_range.contains(&loc.x) && self.y_range.contains(&loc.y) && self.z_range.contains(&loc.z)
    }

    /// Computes the address of a cube in the state, or None 
    /// if the address is out of bounds.
    fn address(&self, loc: &Location) -> Option<usize> {
        if self.in_bounds(loc) {
            let y_stride = range_count(&self.x_range);
            let z_stride = y_stride * range_count(&self.y_range);
            Some(
                ((loc.x - self.x_range.start) as usize) +
                ((loc.y - self.y_range.start) as usize) * y_stride +
                ((loc.z - self.z_range.start) as usize) * z_stride
            )
        } else {
            None
        }
    }

    /// Returns the contents of the cube at the given location.
    fn get(&self, loc: &Location) -> CubeState {
        self.address(loc).map(|a| self.cubes[a]).unwrap_or(CubeState::Inactive)
    }

    /// Sets the contents of a cube.  Panics if the location is out of range.
    fn set(&mut self, loc: &Location, new_state: &CubeState) {
        let a = self.address(loc).unwrap();
        self.cubes[a] = *new_state;
    }

    /// Counts the number of active neighbors of a location
    fn active_neighbors(&self, loc: &Location) -> usize {
        let mut count = 0;
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    if dx != 0 || dy != 0 || dz != 0 {
                        let neighbor = Location {
                            x: loc.x + dx,
                            y: loc.y + dy,
                            z: loc.z + dz,
                        };
                        if self.get(&neighbor) == CubeState::Active {
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }
}

fn run_cycle(prev: &State) -> State {
    // Create a state that's one bigger than the old one.  
    // No new active cube can be more than one step away from
    // an existing one
    let mut result = State::new(
        &extend_range(&prev.x_range),
        &extend_range(&prev.y_range),
        &extend_range(&prev.z_range)
    );
    for x in result.x_range.clone() {
        for y in result.y_range.clone() {
            for z in result.z_range.clone() {
                let loc = Location {x, y, z};
                let old_state = prev.get(&loc);
                let active_count = prev.active_neighbors(&loc);
                let new_state =
                    match old_state {
                        CubeState::Active => 
                            if 2 <= active_count && active_count <= 3 {
                                CubeState::Active
                            } else {
                                CubeState::Inactive
                            },
                        CubeState::Inactive => 
                            if active_count == 3 {
                                CubeState::Active
                            } else {
                                CubeState::Inactive
                            },
                    };
                result.set(&loc, &new_state);
            }
        }
    }
    result
}

fn main() {
    println!("Hello, world!");
}
