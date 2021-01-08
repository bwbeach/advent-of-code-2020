use std::convert::TryInto;
use std::cmp;
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
    (range.end - range.start).try_into().unwrap()
}

// Updates the given range to include the given value.
fn update_range_to_include(range: &mut CoordRange, n: i32) {
    range.start = cmp::min(range.start, n);
    range.end = cmp::max(range.end, n + 1);
}

// Extend a range by one on either end.
fn extend_range(range: &CoordRange) -> CoordRange {
    (range.start - 1) .. (range.end + 1)
}

/// A location in an N-d matrix.  Coordinates can be negative.
#[derive(Clone, Debug, PartialEq)]
struct Location {
    coords: Vec<i32>,
}

/// A rectangular volume in a 3-d matrix.  Coordinates can be negative
#[derive(Clone, Debug, PartialEq)]
struct Volume {
    ranges: Vec<CoordRange>,
}

impl Volume {
    fn contains(&self, loc: &Location) -> bool {
        self.ranges.iter().zip(loc.coords.iter()).all(
            |(r, c)| r.contains(c)
        )
    }

    fn extend_by_one(&self) -> Volume {
        Volume {
            ranges: self.ranges.iter().map(|r| extend_range(r)).collect()
        }
    }

    fn update_to_include(&mut self, loc: &Location) {
        for (r, c) in self.ranges.iter_mut().zip(loc.coords.iter()) {
            update_range_to_include(r, *c)
        }
    }
}

impl IntoIterator for &Volume {
    type Item = Location;
    type IntoIter = VolumeIter;

    fn into_iter(self) -> VolumeIter {
        let ranges = &self.ranges;
        let mut coords: Vec<i32> = ranges.into_iter().map(|r| r.start).collect();
        // When next is called, it will increment first, so we decrement by one
        // so the first value returned will be the right one.
        coords[0] -= 1;

        VolumeIter {
            ranges: self.ranges.clone(),
            current: Location { coords }
        }
    }
}

struct VolumeIter {
    ranges: Vec<CoordRange>,   // TODO: ref
    current: Location,
}

impl Iterator for VolumeIter {
    type Item = Location;

    fn next(&mut self) -> Option<Location> {
        let n = self.ranges.len();
        let at_end = (0..n).into_iter().all(|i| self.current.coords[i] == self.ranges[i].end - 1);
        if at_end {
            None
        } else {
            for i in 0..n {
                if self.current.coords[i] == self.ranges[i].end - 1 {
                    self.current.coords[i] = self.ranges[i].start;
                    // fall through to increment the next counter
                } else {
                    self.current.coords[i] += 1;
                    break;
                }
            }
            Some(self.current.clone())
        }
    }
}

/// Holds the state of the pocket dimension, for a specified
/// span of locations.
/// 
/// TODO: equality should ignore inactive cubes and compare only size, not capacity
#[derive(Debug, PartialEq)]
struct State {
    /// The shape of the matrix this State stores.
    capacity: Volume,

    /// The subset of `capacity` that contains Active cubes
    size: Option<Volume>,

    /// All of the cubes in this State.
    cubes: Vec<CubeState>,
}

impl State {

    /// Creates a new state of the given location and size, with all of the
    /// cubes being inactive.
    fn new(capacity: &Volume) -> State {

        let cube_count = 
            capacity.ranges.iter()
                .map(|r| range_count(r))
                .product();

        State {
            capacity: capacity.clone(),
            size: None,
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
            let mut result = 0;
            let mut stride = 1usize;
            for (r, c) in self.capacity.ranges.iter().zip(loc.coords.iter()) {
                result += ((c - r.start) as usize) * stride;
                stride *= range_count(r);
            }
            Some(result)
        } else {
            None
        }
    }

    /// Returns the contents of the cube at the given location.
    fn get(&self, loc: &Location) -> CubeState {
        self.address(loc).map(|a| self.cubes[a]).unwrap_or(CubeState::Inactive)
    }

    /// Returns the location for a given x and y, with the rest of the
    /// coordinates being 0.
    fn x_y_loc(&self, x: i32, y: i32) -> Location {
        let mut coords = [0i32].repeat(self.capacity.ranges.len());
        coords[0] = x;
        coords[1] = y;
        Location { coords }
    }

    /// Sets the contents of a cube.  Panics if the location is out of range.
    fn set(&mut self, loc: &Location, new_state: &CubeState) {
        if self.get(loc) != CubeState::Inactive {
            panic!("Setting a cube that is already active");
        }
        // Update the size, if needed.
        if *new_state == CubeState::Active {
            match &mut self.size {
                None => {
                    self.size = Some(
                        Volume {
                            ranges: loc.coords.iter().map(|c| (*c)..(c+1)).collect()
                        }
                    );
                },
                Some(volume) => {
                    volume.update_to_include(loc);
                }
            }
        }

        // Store the cube
        let a = self.address(loc).unwrap();
        self.cubes[a] = *new_state;
    }

    /// Counts the number of active neighbors of a location
    fn active_neighbors(&self, loc: &Location) -> usize {
        let mut count = 0;
        // TODO
        // for dx in -1..=1 {
        //     for dy in -1..=1 {
        //         for dz in -1..=1 {
        //             if dx != 0 || dy != 0 || dz != 0 {
        //                 let neighbor = Location {
        //                     x: loc.x + dx,
        //                     y: loc.y + dy,
        //                     z: loc.z + dz,
        //                 };
        //                 if self.get(&neighbor) == CubeState::Active {
        //                     count += 1;
        //                 }
        //             }
        //         }
        //     }
        // }
        count
    }

    /// Counts the number of active cubes in the entire state
    fn count_active(&self) -> usize {
        self.cubes.iter().filter(|c| **c == CubeState::Active).count()
    }
}

fn run_cycle(prev: &State) -> State {
    // Create a state that's one bigger than the old one.  
    // No new active cube can be more than one step away from
    // an existing one
    let prev_size = prev.size.as_ref().unwrap();
    let new_capacity = prev_size.extend_by_one();
    let mut result = State::new(&new_capacity);

    // Compute the state of each cube in the new State
    // TODO
    // for x in new_capacity.x.clone() {
    //     for y in new_capacity.y.clone() {
    //         for z in new_capacity.z.clone() {
    //             let loc = Location {x, y, z};
    //             let old_state = prev.get(&loc);
    //             let active_count = prev.active_neighbors(&loc);
    //             let new_state =
    //                 match old_state {
    //                     CubeState::Active => 
    //                         if 2 <= active_count && active_count <= 3 {
    //                             CubeState::Active
    //                         } else {
    //                             CubeState::Inactive
    //                         },
    //                     CubeState::Inactive => 
    //                         if active_count == 3 {
    //                             CubeState::Active
    //                         } else {
    //                             CubeState::Inactive
    //                         },
    //                 };
    //             // println!("AAA {:?} {:?} {:?} {:?} {:?} {:?}", x, y, z, old_state, active_count, new_state);
    //             result.set(&loc, &new_state);
    //         }
    //     }
    // }
    result
}

fn parse_initial_state(text: &str) -> State {
    let lines: Vec<&str> = text.split("\n").filter(|l| ! l.is_empty()).collect();
    let col_count = lines[0].len() as i32;
    let row_count = lines.len() as i32;
    let capacity = Volume {
        ranges: vec![0..col_count, 0..row_count, 0..1]
    };
    println!("capacity: {:?}", capacity);
    let mut result = State::new(&capacity);
    for (y, line) in (&lines).iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                let loc = result.x_y_loc(x as i32, y as i32);
                result.set(&loc, &CubeState::Active);
            }
        }
    }
    result
}

fn print_state(state: &State) {
    let size = state.size.as_ref().unwrap();
    println!("size = {:?}", size);
    // TODO
    // for z in size.z.clone() {
    //     println!("z={}", z);
    //     for y in size.y.clone() {
    //         for x in size.x.clone() {
    //             let loc = Location{x, y, z};
    //             let s = state.get(&loc);
    //             let c = if s == CubeState::Active { '#' } else { '.' };
    //             print!("{}", c);
    //         }
    //         println!("");
    //     }
    //     println!("");
    // }
}

const TEST_STATE: &str = "
.#.
..#
###
";

const MY_INPUT: &str = "
...###.#
#.#.##..
.##.##..
..##...#
.###.##.
.#..##..
.....###
.####..#
";

fn run_one(initial: &str) {
    let mut state = parse_initial_state(initial);
    println!("Initial state:");
    print_state(&state);

    for cycle in 1..7 {
        state = run_cycle(&state);
        println!("After cycle {:?}:", cycle);
        print_state(&state);
    }
    println!("Active cell total: {:?}", state.count_active())
}

fn main() {
    
    {
        let volume = Volume { ranges: vec![0..2, 2..4, 4..6] };
        let iter_result: Vec<Vec<i32>> = volume.into_iter().map(|loc| loc.coords).collect();
        assert_eq!(
            vec![
                vec![0, 2, 4],
                vec![1, 2, 4],
                vec![0, 3, 4],
                vec![1, 3, 4],
                vec![0, 2, 5],
                vec![1, 2, 5],
                vec![0, 3, 5],
                vec![1, 3, 5],
            ],
            iter_result
        )
    }

    {
        let initial = parse_initial_state(TEST_STATE);
        let mut expected = State::new(&Volume { ranges: vec![0..3, 0..3, 0..1] });
        expected.set(&expected.x_y_loc(1, 0), &CubeState::Active);
        expected.set(&expected.x_y_loc(2, 1), &CubeState::Active);
        expected.set(&expected.x_y_loc(0, 2), &CubeState::Active);
        expected.set(&expected.x_y_loc(1, 2), &CubeState::Active);
        expected.set(&expected.x_y_loc(2, 2), &CubeState::Active);
        assert_eq!(initial, expected);
    }

    // run_one(TEST_STATE);
    // run_one(MY_INPUT);
}
