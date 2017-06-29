
#[allow(dead_code)]
pub fn run() {
    let mut primes: Vec<u64> = vec!(2);
    let mut nth_prime = 1;
    let mut current_number = 3;

    while nth_prime < 10_001 {
        let is_prime = primes.iter().fold(true, |acc, &x| acc && current_number % x != 0);
        if is_prime {
            println!("is prime: {}", current_number);
            nth_prime += 1;
            primes.push(current_number);
        }
        current_number += 1;
    }

    println!("{}", primes.last().unwrap());
}
