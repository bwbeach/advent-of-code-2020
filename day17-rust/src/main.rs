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

/// A rectangular volume in a 3-d matrix.  Coordinates can be negative
#[derive(Clone, Debug)]
struct Volume {
    x: CoordRange,
    y: CoordRange,
    z: CoordRange
}

impl Volume {
    fn contains(&self, loc: &Location) -> bool {
        self.x.contains(&loc.x) && self.y.contains(&loc.y) && self.z.contains(&loc.z)
    }

    fn extend(&self) -> Volume {
        Volume {
            x: extend_range(&self.x),
            y: extend_range(&self.y),
            z: extend_range(&self.z),
        }
    }
}

/// Holds the state of the pocket dimension, for a specified
/// span of locations.
#[derive(Debug)]
struct State {
    /// The shape of the matrix this State stores.
    capacity: Volume,

    /// All of the cubes in this State.
    cubes: Vec<CubeState>,
}

impl State {

    /// Creates a new state of the given location and size, with all of the
    /// cubes being inactive.
    fn new(capacity: &Volume) -> State {

        let cube_count = 
                range_count(&capacity.x)
                    .checked_mul(range_count(&capacity.y)).unwrap()
                    .checked_mul(range_count(&capacity.z)).unwrap();

        State {
            capacity: capacity.clone(),
            cubes: vec![CubeState::Inactive; cube_count]
        }
    }

    /// True iff the given location is within the space of this state.
    fn in_bounds(&self, loc: &Location) -> bool {
        self.capacity.contains(loc)
    }

    /// Computes the address of a cube in the state, or None 
    /// if the address is out of bounds.
    fn address(&self, loc: &Location) -> Option<usize> {
        if self.in_bounds(loc) {
            let y_stride = range_count(&self.capacity.x);
            let z_stride = y_stride * range_count(&self.capacity.y);
            Some(
                ((loc.x - self.capacity.x.start) as usize) +
                ((loc.y - self.capacity.y.start) as usize) * y_stride +
                ((loc.z - self.capacity.z.start) as usize) * z_stride
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
    let mut result = State::new(&prev.capacity.extend());
    for x in result.capacity.x.clone() {
        for y in result.capacity.y.clone() {
            for z in result.capacity.z.clone() {
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
