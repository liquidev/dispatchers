pub fn factorial(n: u32) -> u32 {
    let mut x = 1;
    for i in 1..=n {
        x *= i;
    }
    x
}

pub fn run() -> u32 {
    factorial(10)
}
