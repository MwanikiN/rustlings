#![allow(clippy::ptr_arg)]

// TODO: Fix the compiler errors without changing anything except adding or
// removing references (the character `&`).

// Shouldn't take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase(); //removed the reference to comply with the ownership rules

    println!("{data}");
}

fn main() {
    let data = "Rust is great!".to_string();

    println!("{}", get_char(&data)); /*added reference to comply with the ownership rules and 
    avoided moving the value of `data` into the function */

    string_uppercase(data);
}
