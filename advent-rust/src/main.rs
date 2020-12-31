
fn next_arrival_time(earliest: i32, bus_id: i32) -> i32 {
    (earliest + bus_id - 1) / bus_id * bus_id
}

fn day13_part1() {
    // let earliest = 939;
    // let buses_note = "7,13,x,x,59,x,31,19";
    let earliest = 1015292;
    let buses_note = "19,x,x,x,x,x,x,x,x,41,x,x,x,x,x,x,x,x,x,743,x,x,x,x,x,x,x,x,x,x,x,x,13,17,x,x,x,x,x,x,x,x,x,x,x,x,x,x,29,x,643,x,x,x,x,x,37,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,23";
    let mut bus_ids : Vec<(i32, i32)> = 
            buses_note
                .split(",")
                .filter(|s| *s != "x")
                .map(|s| s.parse::<i32>().unwrap())
                .map(|bus_id| (next_arrival_time(earliest, bus_id), bus_id,))
                .collect();
    bus_ids.sort();
    let first_bus = bus_ids[0];

    let answer = first_bus.1 * (first_bus.0 - earliest);
    println!("Part 1: {:?}  {:?}", first_bus, answer);
}

fn main() {
    day13_part1();
}
