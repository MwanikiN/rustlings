fn call_me(num: u8) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    // fixed the line below to pass a u8 value to the function call_me
    call_me(3);
}
