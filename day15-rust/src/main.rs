use std::collections::HashMap;

/// Computes the nth (1-based) number in the sequence specified
/// by part 1 of day 15.
fn get_nth_number(initial_seq: &[usize], target_turn: usize) -> usize {
    // a table that maps from number to the most recent turn when it was used
    let mut table: HashMap<usize, usize> = HashMap::new();

    // add the initial (1-based) turns to the table, except
    // for the most recent one.
    for turn in 1..initial_seq.len() {
        table.insert(initial_seq[turn - 1], turn);
    }

    // run all of the turns up until the one we want the answer for
    let mut current: usize = *initial_seq.last().unwrap();
    for turn in (initial_seq.len() + 1)..(target_turn + 1) {
        let prev = current;
        match table.get(&current) {
            Option::None => {
                current = 0;
            }
            Option::Some(prev_turn) => {
                current = (turn - 1) - prev_turn;
            }
        }
        table.insert(prev, turn - 1);
    }
    current
}


fn main() {
    assert_eq!(get_nth_number(&[0, 3, 6], 4), 0);
    assert_eq!(get_nth_number(&[0, 3, 6], 5), 3);
    assert_eq!(get_nth_number(&[0, 3, 6], 6), 3);
    assert_eq!(get_nth_number(&[0, 3, 6], 7), 1);
    assert_eq!(get_nth_number(&[0, 3, 6], 8), 0);
    assert_eq!(get_nth_number(&[0, 3, 6], 9), 4);
    assert_eq!(get_nth_number(&[0, 3, 6], 10), 0);
    assert_eq!(get_nth_number(&[0, 3, 6], 2020), 436);
    assert_eq!(get_nth_number(&[1, 3, 2], 2020), 1);
    assert_eq!(get_nth_number(&[2, 1, 3], 2020), 10);
    assert_eq!(get_nth_number(&[1, 2, 3], 2020), 27);
    assert_eq!(get_nth_number(&[2, 3, 1], 2020), 78);
    assert_eq!(get_nth_number(&[3, 2, 1], 2020), 438);
    assert_eq!(get_nth_number(&[3, 1, 2], 2020), 1836);
    println!("Part 1: {:?}", get_nth_number(&[7,12,1,0,16,2], 2020));
    println!("Part 2: {:?}", get_nth_number(&[7,12,1,0,16,2], 30000000));
}
