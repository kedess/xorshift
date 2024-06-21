fn random(seed: u32, n: usize, limit: u32) -> Vec<u32> {
    let mut value = seed;
    std::iter::repeat_with(move || {
        value ^= value << 13;
        value ^= value >> 17;
        value ^= value << 5;
        value % limit
    })
    .take(n)
    .collect()
}

fn main() {
    let list = random(113, 10, u16::MAX as u32);
    println!("{:?}", list);
}
