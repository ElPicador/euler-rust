
#[allow(dead_code)]
pub fn run() {
    let num: u64 = 600_851_475_143;
    let mut factor: u64 = 300_425_737_571;
    let mut found = false;
    let mut i: u64 = 0;

    while !found {
        if num % factor == 0 {
            found = true;
        } else {
            factor -= 2;
        }
        i += 1;
        if i % 1_000_000_000 == 0 {
            println!("{} round", i);
        }
    }

    println!("Biggest factor is {}", factor);
}
