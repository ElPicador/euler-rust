#[allow(dead_code)]
pub fn run() {
    let mut sum = 0;
    let mut n = 1;
    let mut m = 2;
    let mut n_minus_1: u32;

    while m < 4_000_000 {
        if m % 2 == 0 {
            sum += m;
        }

        n_minus_1 = n;
        n = m;
        m = n + n_minus_1;
    }

    println!("The sum is: {}", sum);
}
