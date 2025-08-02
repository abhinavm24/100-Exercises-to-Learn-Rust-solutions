// Rewrite the factorial function using a `for` loop.
pub fn factorial(n: u32) -> u32 {
    let mut result:u32 = 1;
    for i in 2..=n {
        result *= i;
    }
    result
}
