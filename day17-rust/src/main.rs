use std::convert::TryInto;
use std::cmp;
use std::iter::FromIterator;
use std::ops::Index;
use std::ops::IndexMut;
use std::ops::Range;
use streaming_iterator::StreamingIterator;

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

/// The maximum number of dimensions supported
const MAX_DIMENSIONS: usize = 7;

/// A location in an N-d matrix.  Coordinates can be negative.
/// 
/// Coordinates must be in the range -100 to 100.  All coordinates,
/// and the N that is the number of dimensions are stored in a 
/// single u64 value, each one getting one byte.
/// 
/// The low-order byte is N, the number of dimensions, followed
/// by one byte for each of the dimensions, with 0 stored as 0x80,
/// and negative numbers less than that.
/// 
#[derive(Clone, Debug, PartialEq)]
struct Location {
    dimensions: usize,
    data: [i32; MAX_DIMENSIONS],
}

/// A set of coordinates in an N-dimensional space
impl Location {

    /// Returns a new Location with the given X and Y values, and all
    /// other coordinates 0.
    fn new_x_y(x: i32, y: i32, dimensions: usize) -> Location {
        Location {
            dimensions : dimensions,
            data : {
                let mut data = [0i32; MAX_DIMENSIONS];
                data[0] = x;
                data[1] = y;
                data
            }
        }
    }

    fn iter(&self) -> LocIter {
        LocIter { loc : self, next_index: 0 }
    }
}

impl FromIterator<i32> for Location {
    fn from_iter<I: IntoIterator<Item=i32>>(iter: I) -> Self {
        let mut dimensions = 0;
        let mut data = [0i32; MAX_DIMENSIONS];
        for c in iter {
            data[dimensions as usize] = c;
            dimensions += 1;
        }
        Location { dimensions, data }
    }
}

impl Index<usize> for Location {
    type Output = i32;

    fn index(&self, i: usize) -> &Self::Output {
        assert!(i < self.dimensions);
        & self.data[i]
    }
}

impl IndexMut<usize> for Location {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        assert!(i < self.dimensions);
        &mut self.data[i]
    }
}

struct LocIter<'a> {
    loc: &'a Location,
    next_index: usize,
}

impl<'a> Iterator for LocIter<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        if self.next_index < self.loc.dimensions as usize {
            let result = self.loc[self.next_index];
            self.next_index += 1;
            Some(result)
        } else {
            None
        }
    }
}

/// A rectangular volume in a 3-d matrix.  Coordinates can be negative
#[derive(Clone, Debug, PartialEq)]
struct Volume {
    ranges: Vec<CoordRange>,
}

impl Volume {
    fn contains(&self, loc: &Location) -> bool {
        (0..self.ranges.len()).into_iter().all(
            |i| self.ranges[i].contains(&loc[i])
        )
    }

    fn extend_by_one(&self) -> Volume {
        Volume {
            ranges: self.ranges.iter().map(|r| extend_range(r)).collect()
        }
    }

    fn update_to_include(&mut self, loc: &Location) {
        for i in 0..self.ranges.len() {
            update_range_to_include(&mut self.ranges[i], loc[i])
        }
    }

    fn streaming_iter(&self) -> VolumeIter {
        VolumeIter {
            ranges: self.ranges.clone(),  // TODO: ref
            current: {
                let ranges = &self.ranges;
                let mut result: Location = ranges.into_iter().map(|r| r.start).collect();
                // When next is called, it will increment first, so we decrement by one
                // so the first value returned will be the right one.
                result[0] -= 1;
                result
            },
            done: false,
        }
    }
}

struct VolumeIter {
    ranges: Vec<CoordRange>,   // TODO: ref
    current: Location,
    done: bool,
}

impl StreamingIterator for VolumeIter {
    type Item = Location;

    fn advance(&mut self) {
        assert!(! self.done);
        let n = self.ranges.len();
        for i in 0..n {
            if self.current[i] == self.ranges[i].end - 1 {
                self.current[i] = self.ranges[i].start;
                // fall through to increment the next counter
            } else {
                self.current[i] += 1;
                return;
            }
        }
        self.done = true;
    }

    fn get(&self) -> Option<&Self::Item> {
        if self.done {
            None
        } else {
            Some(&self.current)
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
            for (i, r) in self.capacity.ranges.iter().enumerate() {
                let c = loc[i];
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

    // How many dimensions does this state have?
    fn dimensions(&self) -> usize {
        self.capacity.ranges.len()
    }

    /// Returns the location for a given x and y, with the rest of the
    /// coordinates being 0.
    fn x_y_loc(&self, x: i32, y: i32) -> Location {
        Location::new_x_y(x, y, self.dimensions())
    }

    /// Sets the contents of a cube.  Panics if the location is out of range.
    fn set_active(&mut self, loc: &Location) {
        if self.get(loc) != CubeState::Inactive {
            panic!("Setting a cube that is already active");
        }
        // Update the size, if needed.
        match &mut self.size {
            None => {
                self.size = Some(
                    Volume {
                        ranges: (0..self.dimensions()).into_iter().map(|i| (loc[i])..(loc[i]+1)).collect()
                    }
                );
            },
            Some(volume) => {
                volume.update_to_include(loc);
            }
        }

        // Store the cube
        let a = self.address(loc).unwrap();
        self.cubes[a] = CubeState::Active;
    }

    /// Counts the number of active neighbors of a location
    fn active_neighbors(&self, middle: &Location) -> usize {
        // Create a Volume of all of the neighboring cubes, plus the middle one
        let to_check = Volume {
            ranges: middle.iter().map(|n| (n-1) .. (n+2)).collect()
        };

        // Count the active cubes that aren't the middle one.
        to_check.streaming_iter()
            .filter(|loc| loc != middle)
            .filter(|loc| self.get(loc) == CubeState::Active)
            .count()
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

    let mut si = new_capacity.streaming_iter();
    while let Some(loc) = si.next() {
        let old_state = prev.get(&loc);
        let active_count = prev.active_neighbors(&loc);
        let is_active =
            match old_state {
                CubeState::Active => (2 <= active_count && active_count <= 3),
                CubeState::Inactive => (active_count == 3),
            };
        if is_active {
            result.set_active(&loc);
        }
    }

    result
}

fn parse_initial_state(text: &str, dimensions: usize) -> State {
    let lines: Vec<&str> = text.split("\n").filter(|l| ! l.is_empty()).collect();
    let col_count = lines[0].len() as i32;
    let row_count = lines.len() as i32;
    let ranges = {
        let mut ranges = vec![0..col_count, 0..row_count];
        for _ in 2..dimensions {
            ranges.push(0..1);
        }
        ranges
    };
    let capacity = Volume { ranges };
    println!("capacity: {:?}", capacity);
    let mut result = State::new(&capacity);
    for (y, line) in (&lines).iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                let loc = result.x_y_loc(x as i32, y as i32);
                result.set_active(&loc);
            }
        }
    }
    result
}

/// Prints out a State in the format used on the web site
fn print_state(state: &State) {
    let size = state.size.as_ref().unwrap();
    println!("size = {:?}", size);
    let mut prev_z = size.ranges[2].start - 1;  // a Z value than is never used, and won't match the first one
    let mut si = size.streaming_iter();
    while let Some(loc) = si.next() {
        if prev_z != loc[2] {
            println!("AAA {:?}", loc);
        }
        let s = state.get(&loc);
        let c = if s == CubeState::Active { '#' } else { '.' };
        print!("{}", c); 

        if loc[0] == size.ranges[0].end - 1 {
            println!();
            if loc[1] == size.ranges[1].end - 1 {
                println!();
            }
        }
        prev_z = loc[2];
    }
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

fn run_one(initial: &str, dimensions: usize) {
    let mut state = parse_initial_state(initial, dimensions);
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
        let mut iter_result: Vec<Vec<i32>> = Vec::new();
        let mut si = volume.streaming_iter();
        while let Some(loc) = si.next() {
            iter_result.push(loc.iter().collect());
        }
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
        let initial = parse_initial_state(TEST_STATE, 3);
        let mut expected = State::new(&Volume { ranges: vec![0..3, 0..3, 0..1] });
        expected.set_active(&expected.x_y_loc(1, 0));
        expected.set_active(&expected.x_y_loc(2, 1));
        expected.set_active(&expected.x_y_loc(0, 2));
        expected.set_active(&expected.x_y_loc(1, 2));
        expected.set_active(&expected.x_y_loc(2, 2));
        assert_eq!(initial, expected);
    }

    run_one(TEST_STATE, 4);
    run_one(MY_INPUT, 4);

    // part 1: 848
    // part 2: 1980
}
