
/// Generalized implementation of Conway's Game of Life.
/// 
/// The board is a set of cells that are active, named by
/// a type that supports Clone + Eq + Hash, so it can be
/// stored in a hash table.
/// 
/// Cell names must also support Add.  Neighbors of a cell
/// are computed by adding one of a fixed set of offsets
/// to the cell.

use std::collections;
use std::hash;
use std::ops;

/// Counts the number of neighbors of each cell that are alive.
/// 
/// Returns a map from cell to live neigbor count.  Only cells
/// with non-zero counts are included in the set.
fn count_live_neighbors<T: ops::Add<Output = T> + Clone + Eq + hash::Hash>(
    before: &collections::HashSet<T>,
    neighbor_deltas: &[T]
) -> collections::HashMap<T, usize> 
{
    let mut result: collections::HashMap<T, usize> = collections::HashMap::new();
    for c in before.iter() {
        for d in neighbor_deltas.iter() {
            let neighbor = c.clone() + d.clone();
            let old_count = result.get(&neighbor).map(|&n| n).unwrap_or(0);
            result.insert(neighbor, old_count + 1);
        }
    }
    result
}

/// Runs one step in Life.
/// 
/// Input is the board before, and output is the board after.
pub fn conway_step<T: ops::Add<Output = T> + Clone + Eq + hash::Hash>(
    before: &collections::HashSet<T>,
    neighbor_deltas: &[T],
    is_alive: fn(bool, usize) -> bool
) -> collections::HashSet<T> 
{
    // Count the number of neighbors of each cell that are alive.
    let live_neigbors = count_live_neighbors(before, neighbor_deltas);
    
    // Use the aliveness test to decide which cells are alive in 
    // the result.  Any cell that is live in the output must have
    // at least one live neighbor, so it must appear in live_neighbors.
    live_neigbors.iter()
        .filter(|(c, &n)| {
            let was_alive = before.contains(c);
            is_alive(was_alive, n)
        })
        .map(|(c, _)| c.clone())
        .collect() 
}