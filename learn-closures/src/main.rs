
fn map_int(data: &[i32], fcn: &dyn Fn(i32)->i32) -> Vec<i32> {
    let mut result = Vec::new();
    for n in data {
        result.push(fcn(*n));
    }
    result
}


fn main() {
    let two: i32 = 2;
    let data = vec![1, 2, 3];
    let doubled: Vec<_> = data.iter().map(|n: &i32| n * two).collect();
    println!("{:?}", map_int(&data, &|n: i32| n * 2));
    println!("{:?}", doubled);
}
