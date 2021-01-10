use std::collections::HashMap;
use std::collections::HashSet;

type Location = i64;

fn x_y(x: i64, y: i64) -> Location {
    assert!(0 <= x && x < 10 && 0 <= y && y < 10);
    y << 8 | x
}

fn neighbor_offsets(dims: usize) -> Vec<i64> {
    let mut result: Vec<i64> = Vec::new();
    neighbor_offsets_helper(dims, 0, &mut result);
    result
}

fn neighbor_offsets_helper(dims: usize, number_so_far: i64, output: &mut Vec<i64>) {
    if dims == 0 {
        if number_so_far != 0 {
            output.push(number_so_far);
        }
    } else {
        let offset: i64 = 1 << (dims - 1) * 8;
        for i in -1..=1 {
            neighbor_offsets_helper(dims - 1, number_so_far + i * offset, output);
        }
    }
}

fn parse_initial2(bytes: &[u8]) -> HashSet<i64> {
    let mut result = HashSet::new();
    for (y, line) in bytes.split(|&b| b == b'\n').enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == b'#' {
                result.insert(x_y(x as i64, y as i64));
            }
        }
    }
    result
}

/// Parses an initial state, returning the set of active locations.
fn parse_initial(bytes: &[u8]) -> HashSet<i64> {
    bytes
        .split(|&b| b == b'\n')
        .enumerate()
        .flat_map(|(y, line)| line.iter().enumerate().map(move |(x, c)| (x, y, c)))
        .filter(|(_, _, c)| **c == b'#')
        .map(|(x, y, _)| x_y(x as i64, y as i64))
        .collect()
}

/// Reads the input and returns the set of locations that have active cubes.
fn read_input() -> HashSet<i64> {
    parse_initial2(include_bytes!("../input.txt"))
}

fn run_cycle(active_before: &HashSet<i64>, neighbors: &Vec<i64>) -> HashSet<i64> {
    // Count the neighbors of each cell
    let mut counts: HashMap<i64, usize> = HashMap::new();
    for loc in active_before {
        for offset in neighbors {
            let neighbor = loc + offset;
            let entry = counts.entry(neighbor).or_insert(0);
            *entry += 1;
        }
    }

    // Decide the new state of each cell.  The only cells that could possibly
    // be active are the ones that have at least one neighbor and apper in 
    // `counts`.
    counts.into_iter()
        .filter(|(loc, count)| (*count == 3) || (*count == 2 && active_before.contains(&loc)))
        .map(|(loc, _)| loc)
        .collect()
}

fn main() {
    for dims in 3..=4 {
        println!("\n#\n# {:?} dimensions\n#\n", dims);
        let mut state = read_input();
        println!("0: {:?}", state.len());
        for i in 0..6 {
            state = run_cycle(&state, &neighbor_offsets(dims));
            println!("{:?}: {:?}", i + 1, state.len())
        }
    }
}
