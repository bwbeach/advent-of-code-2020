
use std::collections::HashSet;
use std::fmt::Formatter;
use std::fs::read_to_string;

/// Returns the square root of a usize.
/// Panics if the number is not a perfect square.
fn usize_sqrt(n: usize) -> usize {
    let result = (n as f64).sqrt().round() as usize;
    assert_eq!(result * result, n);
    result
}

#[test]
fn test_usize_sqrt() {
    assert_eq!(usize_sqrt(64), 8);
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Edge {
    bits: usize
}

impl Edge {
    fn new(bytes: &[u8]) -> Edge {
        let bits = bytes.iter()
            .enumerate()
            .filter(|(_, &b)| b == b'#')
            .map(|(i, _)| 1usize << i)
            .sum();
        Edge { bits }
    }

    fn from_iter(bytes: EdgeIterator<u8>) -> Edge {
        let bits = bytes
            .enumerate()
            .filter(|&(_, &b)| b == b'#')
            .map(|(i, _)| 1usize << i)
            .sum();
        Edge { bits }
    }
}

#[derive(Clone, Eq, PartialEq)]
struct Tile {
    number: usize,    // which tile is this?
    pixels: Grid<u8>, // the array of pixels in this tile
    top: Edge,        // left-to-right
    right: Edge,      // top-to-bottom
    bottom: Edge,     // left-to-right
    left: Edge,       // top-to-bottom
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        f.write_str("Tile")?;
        self.number.fmt(f)
    }
}
impl Tile {
    fn new(number: usize, pixels: Grid<u8>) -> Tile {
        let top = Edge::from_iter(pixels.top_edge());
        let right = Edge::new(&pixels.right_edge());
        let bottom = Edge::new(&pixels.bottom_edge());
        let left = Edge::new(&pixels.left_edge());
        Tile { number, pixels, top, right, bottom, left }
    }

    fn positions(&self) -> Vec<Tile> {
        self.pixels.positions().into_iter()
            .map(|g| Tile::new(self.number, g))
            .collect()
    }
}

fn parse_tile(text: &str) -> Tile {
    let mut lines_iter = text.split("\n").filter(|s| ! s.is_empty());

    let header = lines_iter.next().unwrap();
    let tile_num = header[5..9].parse::<usize>().unwrap();

    let tile_bytes: Vec<u8> = 
        lines_iter 
            .flat_map(|line| line.as_bytes().iter())
            .map(|&b| b)
            .collect();

    Tile::new(tile_num, Grid::square_from_vec(tile_bytes))
}

#[test]
fn test_parse_tile() {
    assert_eq!(
        parse_tile("Tile 1234:\n##..\n...#\n....\n..#.\n"),
        Tile::new(1234, Grid::square_from_vec(b"##.....#......#.".to_vec()))
    );
}

fn read_input(file_name: &str) -> Vec<Tile> {
    read_to_string(file_name)
        .unwrap()
        .split("\n\n")
        .map(|text| parse_tile(text)) 
        .collect()
}

// struct TileLibrary {
//     orientations: Vec<Tile>
// }

// impl TileLibrary {
//     fn new() -> TileLibrary {
//         TileLibrary {
//             orientations: Vec::new(),
//         }
//     }

//     fn insert(&mut self, tile: &Tile) {
//         for option in tile.positions() {
//             self.orientations.push(option);
//         }
//     }
// }

// struct TileIndex<'a> {
//     left_to_orientation: HashMap<&'a Edge, &'a Tile>,
//     top_to_orientation: HashMap<&'a Edge, &'a Tile>,
// }

// impl<'a> TileIndex<'a> {
//     fn new(orientations: &Vec<Tile>) -> TileIndex {
//         TileIndex {
//             left_to_orientation: orientations.iter().map(|p| (&p.left, p)).collect(),
//             top_to_orientation: orientations.iter().map(|p| (&p.top, p)).collect(),
//         } 
//     }
// }

/// An x-y position within a Grid.
/// 
/// A GridPos is created only by a Grid, which ensures that
/// the coordinates are valid.
#[derive(Copy, Clone)]
struct GridPos {
    x: usize,
    y: usize,
}

impl GridPos {
    fn new(x: usize, y: usize) -> GridPos {
        GridPos{ x, y }
    }
}

/// A square grid of things
#[derive(Clone, Debug, Eq, PartialEq)]
struct Grid<T> {
    width: usize,
    height: usize,

    /// The items in the grid, in row-major order.  The first thing
    /// is the top left.  [size-1] is the top right.
    items: Vec<T>,
}

impl<T: Clone> Grid<T> {
    /// Creates a new grid of the given size, with every element
    /// containing the same value.
    fn new(width: usize, height: usize, initial_value: T) -> Grid<T> {
        let items = vec![initial_value; width * height];
        Grid { width, height, items }
    }

    /// Creates a new grid, with values supplied from a slice of values.
    fn from_vec(width: usize, height: usize, items: Vec<T>) -> Grid<T> {
        assert_eq!(width * height, items.len());
        Grid { width: width, height: height, items }
    }

    /// Creates a new grid, with values supplied from a slice of values.
    fn square_from_vec(items: Vec<T>) -> Grid<T> {
        let size = usize_sqrt(items.len());
        Grid { width: size, height: size, items }
    }

    /// Returns the first cell in the grid, the one at the top left.
    fn first(&self) -> GridPos {
        GridPos::new(0, 0)
    }

    /// Returns the next cell after the given one, in the order
    /// they are filled in: left-to-right, top-to-bottom.
    fn next(&self, p: GridPos) -> Option<GridPos> {
        if p.x < self.width - 1 {
            Some(GridPos::new(p.x + 1, p.y))
        } else if p.y < self.height - 1 {
            Some(GridPos::new(0, p.y + 1))
        } else {
            None
        }
    }

    /// Returns the cell above the given one.
    fn up(&self, p: GridPos) -> Option<GridPos> {
        if 0 < p.y {
            Some(GridPos::new(p.x, p.y - 1))
        } else {
            None
        }
    }

    /// Returns the cell to the left of the given one.
    fn left(&self, p: GridPos) -> Option<GridPos> {
        if 0 < p.x {
            Some(GridPos::new(p.x - 1, p.y))
        } else {
            None
        }
    }

    /// Stores a value in a cell in the grid
    fn set(&mut self, p: GridPos, value: T) {
        self.items[p.x + self.width * p.y] = value;
    }

    /// Stores a value in a cell in the grid
    fn get(&self, p: GridPos) -> &T {
        &self.items[p.x + self.width * p.y]
    }

    /// Return the things at the four corners of the grid
    fn corners(&self) -> Vec<T> {
        vec![
            self.items[0].clone(),
            self.items[self.width - 1].clone(),
            self.items[self.width * (self.height - 1)].clone(),
            self.items[self.width * self.height - 1].clone()
        ]
    }

    // The top edge
    fn top_edge<'a>(&'a self) -> EdgeIterator<'a, T> {
        EdgeIterator::new(&self.items, self.width, 0, 1)
    }

    // The right edge
    fn right_edge(&self) -> Vec<T> {
        (0..self.height).into_iter()
            .map(|i| self.get(GridPos::new(self.width - 1, i)).clone())
            .collect()
    }

    // The bottom edge
    fn bottom_edge(&self) -> Vec<T> {
        (0..self.width).into_iter()
            .map(|i| self.get(GridPos::new(i, self.height - 1)).clone())
            .collect()
    }

    // The left edge
    fn left_edge(&self) -> Vec<T> {
        (0..self.height).into_iter()
            .map(|i| self.get(GridPos::new(0, i)).clone())
            .collect()
    }

    /// Rotates a grid 90 degrees clockwise
    fn rotate(&self) -> Grid<T> {
        let mut elems = Vec::new();
        for y in 0..self.width {
            for x in 0..self.height {
                elems.push(self.get(GridPos::new(y, self.height - x - 1)).clone());
            }
        }
        Grid::from_vec(self.height, self.width, elems)
    }

    /// Flips a grid on its vertial axis
    fn flip(&self) -> Grid<T> {
        let mut elems = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                elems.push(self.get(GridPos::new(self.width - x - 1, y)).clone());
            }
        }
        Grid::from_vec(self.width, self.height, elems)
    }

    /// Returns grids for all rotations and reflections of this one.
    fn positions(&self) -> Vec<Grid<T>> {
        let a = self.clone();
        let b = a.rotate();
        let c = b.rotate();
        let d = c.rotate();
        let e = a.flip();
        let f = e.rotate();
        let g = f.rotate();
        let h = g.rotate();
        vec![a, b, c, d, e, f, g, h]
    }
}

#[test]
fn test_rotate_square() {
    let original : Grid<u8> = 
        Grid::square_from_vec(
            vec![
                1, 0, 0, 5, 
                0, 2, 6, 0, 
                0, 7, 3, 0,
                8, 0, 0, 4
            ]
        );
    let rotated = 
        Grid::square_from_vec(
            vec![
                8, 0, 0, 1, 
                0, 7, 2, 0, 
                0, 3, 6, 0,
                4, 0, 0, 5
            ]
        );
    let flipped =
        Grid::square_from_vec(
            vec![
                5, 0, 0, 1, 
                0, 6, 2, 0, 
                0, 3, 7, 0,
                4, 0, 0, 8
            ]
        );
    
    assert_eq!(*original.get(GridPos::new(0, 0)), 1);
    assert_eq!(*original.get(GridPos::new(3, 0)), 5);
    assert_eq!(*original.get(GridPos::new(2, 1)), 6);
    assert_eq!(original.rotate(), rotated);
    assert_eq!(original.flip(), flipped);
}

#[test]
fn test_rotate_rect() {
    let original : Grid<u8> = 
        Grid::from_vec(
            4, 2,
            vec![
                1, 0, 0, 5, 
                0, 2, 6, 0,
            ]
        );
    let rotated = 
        Grid::from_vec(
            2, 4,
            vec![
                0, 1, 
                2, 0, 
                6, 0,
                0, 5,
            ]
        );
    let flipped =
        Grid::from_vec(
            4, 2,
            vec![
                5, 0, 0, 1, 
                0, 6, 2, 0,
            ]
        );
    
    assert_eq!(*original.get(GridPos::new(0, 0)), 1);
    assert_eq!(*original.get(GridPos::new(3, 0)), 5);
    assert_eq!(*original.get(GridPos::new(2, 1)), 6);
    assert_eq!(original.rotate(), rotated);
    assert_eq!(original.flip(), flipped);
}

struct EdgeIterator<'a, T> {
    range: std::ops::Range<usize>,
    start: usize,
    stride: usize,
    items: &'a Vec<T>,
}

impl<'a, T> EdgeIterator<'a, T> {
    fn new(items: &'a Vec<T>, count: usize, start: usize, stride: usize) -> EdgeIterator<'a, T> {
        let range = 0..count;
        EdgeIterator { range, start, stride, items }
    }
}

impl<'a, T> Iterator for EdgeIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.range.next().map(|i| &self.items[self.start + i * self.stride])
    }
}

fn solve_part1<'a, 'b>(choices: &'a Vec<&'a Tile>, grid: &'b mut Grid<Option<&'a Tile>>, used: &'b mut HashSet<usize>, pos: GridPos) {
    for c in choices {
        if used.contains(&c.number) {
            continue;
        }
        if let Some(left) = grid.left(pos) {
            if grid.get(left).unwrap().right != c.left {
                continue;
            }
        }
        if let Some(up) = grid.up(pos) {
            if grid.get(up).unwrap().bottom != c.top {
                continue;
            }
        }

        grid.set(pos, Some(*c));
        used.insert(c.number);

        if let Some(next_pos) = grid.next(pos) {
            solve_part1(choices, grid, used, next_pos);
        } else {
            let answer: usize = grid.corners().iter().map(|t| t.unwrap().number).product();
            println!("SOLVED!  {:?}", answer);
        }

        grid.set(pos, None);
        used.remove(&c.number);
    }
}

fn part1(file_name: &str) {
    let tiles_from_input = read_input(file_name);
    let size = usize_sqrt(tiles_from_input.len());

    let choices: Vec<Tile> =
        tiles_from_input
            .iter()
            .flat_map(|tile| tile.positions())
            .collect();

    let choice_refs: Vec<&Tile> = choices.iter().collect();

    let mut grid: Grid<Option<&Tile>> = Grid::new(size, size, None);
    let mut used: HashSet<usize> = HashSet::new();
    let first = grid.first();
    let second = grid.next(first).unwrap();

    for &c in choice_refs.iter() {
        used.insert(c.number);
        grid.set(first, Some(c));
        
        solve_part1(&choice_refs, &mut grid, &mut used, second);
        
        used.remove(&c.number);
        grid.set(first, None);
    }
}

fn main() {
    part1("sample1.txt");  // 20899048083289
    part1("input.txt");  // 22878471088273
}
