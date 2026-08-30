fn main() {
    let a = [1, 2, 3, 4, 5];
    let nice_slice = &a[1..4];
    println!("{:?}", nice_slice);
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];

        // TODO: Get a slice called `nice_slice` out of the array `a` so that the test passes.
        let nice_slice = &a[1..4]; // This creates a slice from index 1 to 3 (4 is exclusive)

        assert_eq!([2, 3, 4], nice_slice);
    }
}
