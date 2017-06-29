#[allow(dead_code)]
pub fn run() {
    let mut primes: Vec<u64> = vec![2];

    for i in 3..2_000_001 {
        if i % 2 == 0 {
            continue;
        }
        if i % 1000 == 1 {
            println!("{}", i);
        }
        let mut is_prime = true;
        for j in primes.iter() {
            if i % j == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            primes.push(i);
        }
    }

    let sum = primes.iter().fold(0u64, |acc, &x| acc + x);
    println!("{}", sum);
}
