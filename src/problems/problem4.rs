
#[allow(dead_code)]
pub fn run() {
    let mut values: Vec<u32> = Vec::new();

    for i in 100..999 {
        for j in 100..999 {
            if is_palindrome(i * j) {
                values.push(i * j);
            }
        }
    }
    values.sort_by(|a, b| a.cmp(b));

    println!("{}", values.pop().unwrap());
}

fn is_palindrome(int: u32) -> bool {
    let string_rep: String = String::from(int.to_string());
    let reverse: String = string_rep.chars().rev().collect();
    string_rep == reverse
}
