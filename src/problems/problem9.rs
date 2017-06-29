#[allow(dead_code)]
pub fn run() {
    for a in 0u32..1001 {
        for b in 0u32..1001 {
            for c in 0u32..1001 {
                if a < b && b < c && a + b + c == 1000 && a.pow(2) + b.pow(2) == c.pow(2) {
                    println!("{:?}", a * b * c);
                    return;
                }
            }
        }
    }
}
