// The Rust compiler needs to know how to check whether supplied references are
// valid, so that it can let the programmer know if a reference is at risk of
// going out of scope before it is used. Remember, references are borrows and do
// not own their own data. What if their owner goes out of scope?

// TODO: Fix the compiler error by updating the function signature.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { //the return type should outlive the references passed in. 
    if x.len() > y.len() { //The lifetime of the return type is the same as the lifetime of the input references. This is called a lifetime annotation.
        x //lifetime annotations are a way to tell the compiler how long a reference should be valid for. 
    } else { //In this case, the return type is valid for the same lifetime as the input references.
        y
    }
}

fn main() {
    // You can optionally experiment here.
    println!("The longest string is: {}", longest("abcd", "123"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest() {
        assert_eq!(longest("abcd", "123"), "abcd");
        assert_eq!(longest("abc", "1234"), "1234");
    }
}
