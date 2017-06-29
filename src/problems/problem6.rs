
#[allow(dead_code)]
pub fn run() {
    let range: Vec<u64> = (1..101).collect();
    let sum_of_square = range.iter().fold(0u64, |acc, &x| acc + x.pow(2));
    let square_of_sum: u64 = range.iter().fold(0u64, |acc, &x| acc + x).pow(2);

    println!("{}", square_of_sum - sum_of_square);
}
