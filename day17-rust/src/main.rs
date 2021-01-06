use std::convert::TryInto;

/// The state of one cube in the pocket dimension.
#[derive(Clone, Copy, Debug, PartialEq)]
enum CubeState {
    Active,
    Inactive,
}

/// A location in a 3-d matrix.  Coordinates can be negative.
#[derive(Clone, Debug)]
struct Location {
    x: i32, 
    y: i32, 
    z: i32,
}

/// The size of a 3-d matrix.
#[derive(Clone, Debug)]
struct Size {
    x: usize, 
    y: usize, 
    z: usize,
}

/// Holds the state of the pocket dimension, for a specified
/// span of locations.
#[derive(Debug)]
struct State {
    /// The lower corner (in all three dimensions) of the cubes in this state.
    lower_corner: Location,

    /// The upper corner (in all three dimensions) of the cubes in this state.
    upper_corner : Location,

    /// The size (exclusive) of this state.  The number of cubes
    /// in this state is (size.x * size.y * size.z).  All three
    /// numbers in size must be positive
    size: Size,

    /// The strides to adjacent cubes in each dimension.
    stride: Size,

    /// All of the cubes in this State.
    cubes: Vec<CubeState>,
}

impl State {

    /// Creates a new state of the given location and size, with all of the
    /// cubes being inactive.
    fn new(lower_corner: &Location, size: &Size) -> State {

        let upper_corner = Location{
            x: lower_corner.x + (size.x as i32),
            y: lower_corner.x + (size.y as i32),
            z: lower_corner.x + (size.z as i32),
        };

        let cube_count = size.x.checked_mul(size.y).unwrap().checked_mul(size.z).unwrap();

        let stride = Size {
            x : 1,
            y : size.x,
            z : size.x * size.y
        };

        State {
            lower_corner: lower_corner.clone(),
            upper_corner,
            size: size.clone(),
            stride,
            cubes: vec![CubeState::Inactive; cube_count]
        }
    }

    /// True iff the given location is within the space of this state.
    fn in_bounds(&self, loc: &Location) -> bool {
        self.lower_corner.x <= loc.x && loc.x <= self.upper_corner.x &&
        self.lower_corner.y <= loc.y && loc.y <= self.upper_corner.y &&
        self.lower_corner.z <= loc.z && loc.z <= self.upper_corner.z
    }

    /// Computes the address of a cube in the state, or None 
    /// if the address is out of bounds.
    fn address(&self, loc: &Location) -> Option<usize> {
        if self.in_bounds(loc) {
            Some(
                ((loc.x - self.lower_corner.x) as usize) * self.stride.x +
                ((loc.y - self.lower_corner.y) as usize) * self.stride.y +
                ((loc.z - self.lower_corner.z) as usize) * self.stride.z
            )
        } else {
            None
        }
    }

    /// Returns the contents of the cube at the given location.
    fn get(&self, loc: &Location) -> CubeState {
        self.address(loc).map(|a| self.cubes[a]).unwrap_or(CubeState::Inactive)
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
        &Location {
            x: prev.lower_corner.x - 1,
            y: prev.lower_corner.y - 1,
            z: prev.lower_corner.z - 1,
        },
        &Size {
            x: prev.size.x + 2,
            y: prev.size.y + 2,
            z: prev.size.z + 2,
        }
    );
    result
}

fn main() {
    println!("Hello, world!");
}
