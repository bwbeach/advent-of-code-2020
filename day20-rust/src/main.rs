
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Edge {
    bytes: Vec<u8>,
}

impl Edge {
    fn new(bytes: &[u8]) -> Edge {
        Edge {
            bytes: Vec::from(bytes)
        }
    }

    fn reverse(&self) -> Edge {
        let mut bytes = self.bytes.clone();
        bytes.reverse();
        Edge { bytes }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Tile {
    number: usize, // which tile is this?
    top: Edge,     // left-to-right
    right: Edge,   // top-to-bottom
    bottom: Edge,  // left-to-right
    left: Edge,    // top-to-bottom
}

impl Tile {
    fn rotate(&self) -> Tile {
        Tile {
            number: self.number,
            top: self.left.reverse(),
            right: self.top.clone(),
            bottom: self.right.reverse(),
            left: self.bottom.clone(),
        }
    }

    fn flip(&self) -> Tile {
        Tile {
            number: self.number,
            top: self.top.reverse(),
            right: self.left.clone(),
            bottom: self.bottom.reverse(),
            left: self.right.clone(),
        }
    }

    fn positions(&self) -> Vec<Tile> {
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

fn last_byte(s: &str) -> u8 {
    let bytes = s.as_bytes();
    bytes[bytes.len() - 1]
}

fn parse_tile(text: &str) -> Tile {
    let mut lines = text.split("\n").filter(|s| ! s.is_empty());

    let header = lines.next().unwrap();
    let tile_num = header[5..9].parse::<usize>().unwrap();

    let tile_lines: Vec<_> = lines.collect();

    let top = tile_lines[0].as_bytes();
    let right: Vec<u8> = tile_lines.iter().map(|s| last_byte(s)).collect();
    let left: Vec<u8> = tile_lines.iter().map(|s| s.as_bytes()[0]).collect();
    let bottom = tile_lines[tile_lines.len() - 1].as_bytes();

    Tile {
        number: tile_num,
        top: Edge::new(top),
        right: Edge::new(&right),
        bottom: Edge::new(bottom),
        left: Edge::new(&left)
    }
}

fn read_input(file_name: &str) -> Vec<Tile> {
    read_to_string(file_name)
        .unwrap()
        .split("\n\n")
        .map(|text| parse_tile(text)) 
        .collect()
}

struct TileLibrary {
    orientations: Vec<Tile>
}

impl TileLibrary {
    fn new() -> TileLibrary {
        TileLibrary {
            orientations: Vec::new(),
        }
    }

    fn insert(&mut self, tile: &Tile) {
        for option in tile.positions() {
            self.orientations.push(option);
        }
    }
}

struct TileIndex<'a> {
    left_to_orientation: HashMap<&'a Edge, &'a Tile>,
    top_to_orientation: HashMap<&'a Edge, &'a Tile>,
}

impl<'a> TileIndex<'a> {
    fn new(orientations: &Vec<Tile>) -> TileIndex {
        TileIndex {
            left_to_orientation: orientations.iter().map(|p| (&p.left, p)).collect(),
            top_to_orientation: orientations.iter().map(|p| (&p.top, p)).collect(),
        } 
    }
}

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
struct Grid<T> {
    /// Both the width and the height of the grid.
    size: usize,

    /// The items in the grid, in row-major order.  The first thing
    /// is the top left.  [size-1] is the top right.
    items: Vec<T>,
}

impl<T: Clone> Grid<T> {
    fn new(size: usize, initial_value: T) -> Grid<T> {
        let items = vec![initial_value; size * size];
        Grid { size, items }
    }

    /// Returns the first cell in the grid, the one at the top left.
    fn first(&self) -> GridPos {
        GridPos::new(0, 0)
    }

    /// Returns the next cell after the given one, in the order
    /// they are filled in: left-to-right, top-to-bottom.
    fn next(&self, p: GridPos) -> Option<GridPos> {
        if p.x < self.size - 1 {
            Some(GridPos::new(p.x + 1, p.y))
        } else if p.y < self.size - 1 {
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

    /// Returns the cell above the given one.
    fn left(&self, p: GridPos) -> Option<GridPos> {
        if 0 < p.x {
            Some(GridPos::new(p.x - 1, p.y))
        } else {
            None
        }
    }

    /// Stores a value in a cell in the grid
    fn set(&mut self, p: GridPos, value: T) {
        self.items[p.x + self.size * p.y] = value;
    }

    /// Stores a value in a cell in the grid
    fn get(&self, p: GridPos) -> &T {
        &self.items[p.x + self.size * p.y]
    }

    /// Return the things at the four corners of the grid
    fn corners(&self) -> Vec<T> {
        vec![
            self.items[0].clone(),
            self.items[self.size - 1].clone(),
            self.items[self.size * (self.size - 1)].clone(),
            self.items[self.size * self.size - 1].clone()
        ]
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
    let choices: Vec<Tile> =
        read_input(file_name)
            .iter()
            .flat_map(|tile| tile.positions())
            .collect();

    let choice_refs: Vec<&Tile> = choices.iter().collect();

    let size = ((choices.len() / 8) as f64).sqrt().round() as usize;
    assert_eq!(size * size, choices.len() / 8);

    let mut grid: Grid<Option<&Tile>> = Grid::new(size, None);
    let mut used: HashSet<usize> = HashSet::new();
    let first = grid.first();
    let second = grid.next(first).unwrap();

    for c in choice_refs.iter() {
        used.insert(c.number);
        grid.set(first, Some(&c));
        
        solve_part1(&choice_refs, &mut grid, &mut used, second);
        
        used.remove(&c.number);
        grid.set(first, None);
    }
}

fn main() {
    let edge1 = Edge::new("#.#".as_bytes());
    assert_eq!(edge1, edge1.reverse());
    assert_eq!(Edge::new(b"#.#...").reverse().bytes, b"...#.#");
    assert_eq!(
        parse_tile("Tile 1234:\n##..\n...#\n....\n..#.\n"),
        Tile {
            number: 1234,
            top: Edge::new(b"##.."),
            right: Edge::new(b".#.."),
            bottom: Edge::new(b"..#."),
            left: Edge::new(b"#..."),
        }
    );
    println!("Hello, world!");

    part1("sample1.txt");  // 20899048083289
    part1("input.txt");  // 22878471088273
}
