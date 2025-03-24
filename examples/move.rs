fn main() {
    // let data: u8 = 28;

    // move || println!("im {}", data);

    let data = vec![1, 2, 3];
    let closure = move || println!("captured {data:?} by value");

    closure();
}
