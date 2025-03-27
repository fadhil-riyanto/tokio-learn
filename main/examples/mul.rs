#[deny(arithmetic_overflow)]
fn main() {
    let data: u128 = 192168433812700121474836470895391634433 * 2;

    println!("{}", data);
}