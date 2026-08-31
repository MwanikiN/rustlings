fn main() {
    let mut x = Vec::new(); 
    let y = &mut x;
    y.push(42);
    let z = &mut x;
    z.push(13);
    println!("{:?}", x);

}

#[cfg(test)]
mod tests {
    // TODO: Fix the compiler errors only by reordering the lines in the test.
    // Don't add, change or remove any line.
    #[test]
    fn move_semantics4() {
        let mut x = Vec::new();
        let y = &mut x;
        y.push(42); //moved this push to comply with the move rules
        let z = &mut x;
        z.push(13);
        assert_eq!(x, [42, 13]);
    }
}
