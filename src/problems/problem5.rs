
#[allow(dead_code)]
pub fn run() {
    let mult: u64 = 2 * 3 * 2 * 5 * 7 * 2 * 3 * 11 * 13 * 2 * 17 * 19;
    let mut found = false;
    let mut i = mult - 1;

    while !found  {
        i += 1;
        if i % mult == 0 {
            found = true;
        }
    }

    println!("{}", i);
}
