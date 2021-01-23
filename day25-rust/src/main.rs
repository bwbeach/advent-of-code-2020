
fn transform(subject_number: usize, loop_size: usize) -> usize {
    let mut value = 1;
    for _ in 0..loop_size {
        value = value * subject_number;
        value = value % 20201227;
    }
    value
}

fn find_loop_size(subject_number: usize, public_key: usize) -> usize {
    let mut loop_size = 0;
    let mut value = 1;
    loop {
        if value == public_key {
            return loop_size
        }
        value = value * subject_number;
        value = value % 20201227;
        loop_size += 1;
    }
}

#[test]
fn test_find_loop_size() {
    assert_eq!(find_loop_size(7, 5764801), 8);
    assert_eq!(find_loop_size(7, 17807724), 11);
}

fn find_encryption_key(public_key_1: usize, public_key_2: usize) -> usize {
    let loop_size_1 = find_loop_size(7, public_key_1);
    transform(public_key_2, loop_size_1)
}

#[test]
fn test_find_encryption_key() {
    assert_eq!(find_encryption_key(5764801, 17807724), 14897079);
    assert_eq!(find_encryption_key(17807724, 5764801), 14897079);
}

fn main() {
    println!("Part 1: {:?}", find_encryption_key(12578151, 5051300));
}
