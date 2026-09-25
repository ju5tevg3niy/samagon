unsafe extern "C" {
    fn add(left: u64, right: u64) -> u64;
}

fn main() {
    let result = unsafe { add(12345, 1000000000000) };

    println!("Hello world! C add result: {result}")
}
