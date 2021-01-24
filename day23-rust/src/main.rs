
use std::collections::HashMap;
use std::fmt;
use std::fmt::Debug;
use std::hash::Hash;
use std::iter::FromIterator;

/// A circle of things.
/// 
/// This implementation keeps two copies of each thing,
/// and expects the things to have the Copy trait.
/// 
#[derive(Clone)]
struct Ring<T: Copy + Debug + Eq + Hash> {

    // Map from one item to the thing on its right.
    // Always exactly one entry for each thing in the ring.
    right: HashMap<T, T>,

    // The currently selected item, or None if there's nothing in the Ring.
    current: Option<T>,
}

impl<T: Copy + Debug + Eq + Hash> Ring<T> {

    fn new() -> Ring<T> {
        Ring { right: HashMap::new(), current: None }
    }

    // Does the ring contain this value?
    fn contains(&self, item: T) -> bool {
        self.right.contains_key(&item)
    }

    // Adds the first item to the ring, which becomes
    // the current item.
    fn add_first(&mut self, item: T) {
        assert!(self.current == None);
        self.right.insert(item, item);
        self.current = Some(item);
    }

    // Adds a new item t othe right of the given item
    fn add_right(&mut self, item: T, reference: T) {
        let neighbor = *self.right.get(&reference).unwrap();
        self.right.insert(reference, item);
        self.right.insert(item, neighbor);
    }

    // Removes the item to the right of the given item, and returns it.
    fn remove_right(&mut self, item: T) -> T {
        let right = *self.right.get(&item).unwrap();
        let right_right = *self.right.get(&right).unwrap();
        self.right.insert(item, right_right);
        self.right.remove(&right);
        right
    }

    // Sets the current cup
    fn set_current(&mut self, item: T) {
        assert!(self.right.contains_key(&item));
        self.current = Some(item);
    }

    // Moves the current cup one to the right
    fn move_current_right(&mut self) {
        let current = self.current.unwrap();
        let next = *self.right.get(&current).unwrap();
        self.set_current(next);
    }

    // Returns an iterator over the things in the ring, starting
    // with the current item.
    fn iter<'a>(&'a self) -> RingIterator<'_, T> {
        RingIterator {
            ring: self,
            start: self.current,
            next: self.current,
        }
    }
}

impl<T: Copy + Debug + Eq + Hash> FromIterator<T> for Ring<T> {
    fn from_iter<I: IntoIterator<Item=T>>(into_iterator: I) -> Self {
        let mut ring = Ring::new();
        let mut iter = into_iterator.into_iter();
        if let Some(first) = iter.next() {
            ring.add_first(first);
            let mut prev = first;
            for item in iter {
                ring.add_right(item, prev);
                prev = item;
            }
        }

        ring
    }
}

impl<T: Copy + Debug + Eq + Hash> Debug for Ring<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for item in self.iter() {
            f.write_fmt(format_args!("{:?}", item))?;
        }
        Ok(())
    }
}

struct RingIterator<'a, T: Copy + Debug + Eq + Hash> {
    ring: &'a Ring<T>,
    start: Option<T>,
    next: Option<T>,
}

impl<'a, T: Copy + Debug + Hash + Eq> Iterator for RingIterator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let result = self.next;
        if let Some(n) = self.next {
            let next_next = *self.ring.right.get(&n).unwrap();
            self.next = 
                if next_next == self.start.unwrap() {
                    None
                } else {
                    Some(next_next)
                }
        }
        result
    }
}

#[test]
fn test_ring_iter() {
    let vec: Vec<usize> = vec![2, 3, 5, 7];
    let ring: Ring<usize> = vec.iter().map(|&n| n).collect();
    let new_vec: Vec<usize> = ring.iter().collect();
    assert_eq!(vec, new_vec);
}

fn ring_from_str(s: &str) -> Ring<usize> {
    s.chars().map(|c| c.to_digit(10)).map(|n| n.unwrap() as usize).collect()
}

fn pick_destination(ring: &Ring<usize>) -> usize {
    let mut candidate = ring.current.unwrap() - 1;
    while 0 < candidate {
        if ring.contains(candidate) {
            return candidate;
        }
        candidate -= 1;
    }
    ring.iter().max().unwrap()
}

fn one_step(ring: &mut Ring<usize>) {
    // what's the current cup?
    let current = ring.current.unwrap();

    // remove the three cups to the right of the current cup
    let a = ring.remove_right(current);
    let b = ring.remove_right(current);
    let c = ring.remove_right(current);

    // pick the destination cup
    let destination = pick_destination(&ring);

    // add the three cups picked up after the destination
    ring.add_right(c, destination);
    ring.add_right(b, destination);
    ring.add_right(a, destination);

    // set the new current cup
    ring.move_current_right();
}

fn cups_after_one(ring: &Ring<usize>) -> String {
    let mut tmp = ring.clone();
    tmp.set_current(1);
    tmp.iter().skip(1).map(|n| n.to_string()).collect()
}

fn run_part1(start: &Ring<usize>) -> String {
    let mut work = start.clone();
    for _ in 0..100 {
        one_step(&mut work);
    }
    cups_after_one(&work)
}

fn run_part2(start: &Ring<usize>) -> usize {
    let mut work = 
        start.iter()
            .chain(10..=1000000)
            .collect();
    for _ in 0..10000000 {
        one_step(&mut work);
    }
    let a = work.remove_right(1);
    let b = work.remove_right(1);
    a * b
}

fn main() {
    let sample = ring_from_str("389125467");
    assert_eq!(run_part1(&sample), "67384529");
    
    let input = ring_from_str("952438716");
    let part1_answer = run_part1(&input);
    assert_eq!(part1_answer, "97342568");  // verified on web site
    println!("Part 1: {:?}", part1_answer);

    let part2_answer = run_part2(&input);
    assert_eq!(part2_answer, 902208073192);  // verified on web site
    println!("Part 2: {:?}", part2_answer);
}
