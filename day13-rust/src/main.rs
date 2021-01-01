

const INPUT_EARLIEST: u64 = 1015292;
const INPUT_STR: &str = "19,x,x,x,x,x,x,x,x,41,x,x,x,x,x,x,x,x,x,743,x,x,x,x,x,x,x,x,x,x,x,x,13,17,x,x,x,x,x,x,x,x,x,x,x,x,x,x,29,x,643,x,x,x,x,x,37,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,23";

struct Input {
    earliest: i32,
    buses_note: String,
}

fn next_arrival_time(earliest: i32, bus_id: i32) -> i32 {
    (earliest + bus_id - 1) / bus_id * bus_id
}

fn day13_part1(label: &str, input: Input) {
    let mut bus_ids : Vec<(i32, i32)> = 
            input.buses_note
                .split(",")
                .filter(|s| *s != "x")
                .map(|s| s.parse::<i32>().unwrap())
                .map(|bus_id| (next_arrival_time(input.earliest, bus_id), bus_id,))
                .collect();
    bus_ids.sort();
    let first_bus = bus_ids[0];

    let answer = first_bus.1 * (first_bus.0 - input.earliest);
    println!("{:?}: {:?}  {:?}", label, first_bus, answer);
}

struct Bus {
    id: u64,
    index: u64,
}

/// Takes a string that is either "x" or an integer, and returns
/// the corresponding Bus spec.
fn parse_bus(bus_str: &str, index: u64) -> Option<Bus> {
    if bus_str == "x" {
        Option::None
    } else {
        let id = bus_str.parse::<u64>().unwrap();
        Option::Some(Bus{id: id, index: index})
    }
}

/// Takes a common-separated list of buses and returns a 
/// vector of Bus specs.
fn parse_buses(input: &str) -> Vec<Bus> {
    let strings: Vec<&str> = input.split(",").collect();
    let count = strings.len() as u64;
    let mut result: Vec<Bus> = vec![];
    for i in 0..count {
        let maybe_bus = parse_bus(strings[i as usize], i);
        if maybe_bus.is_some() {
            result.push(maybe_bus.unwrap())
        }
    };
    result
}

fn solve_part2(start: u64, step: u64, buses: &[Bus]) -> u64 {
    match buses.split_first() {
        Option::None => start,
        Option::Some((first, rest,)) => {
            for i in 0..first.id {
                let candidate = start + i * step;
                if (candidate + first.index) % first.id == 0 {
                    return solve_part2(candidate, step * first.id, rest)
                }
            };
            0
        }
    }
}

fn day13_part2(input: &str) -> u64 {
    let buses = parse_buses(input);
    solve_part2(0, 1, &buses)
}

fn main() {
    day13_part1(
        "Part 1 test",
        Input{
            earliest: 939,
            buses_note: String::from("7,13,x,x,59,x,31,19")
        }
    );
    day13_part1(
        "Part 1",
        Input {
            earliest: INPUT_EARLIEST as i32,
            buses_note: String::from(INPUT_STR),
        }
    );
    assert_eq!(14, day13_part2("7,5"));
    assert_eq!(3417, day13_part2("17,x,13,19"));
    assert_eq!(754018, day13_part2("67,7,59,61"));
    assert_eq!(779210, day13_part2("67,x,7,59,61"));
    assert_eq!(1261476, day13_part2("67,7,x,59,61"));
    assert_eq!(1202161486, day13_part2("1789,37,47,1889"));
    println!("Part 2: {:?}", day13_part2(INPUT_STR));
}
