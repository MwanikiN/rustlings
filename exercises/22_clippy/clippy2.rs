fn main() {
    let mut res = 42;
    let option = Some(12);
    // TODO: Fix the Clippy lint.
    // for x in option {
    //     res += x;
    // } changing this block to use if let instead
    if let Some(x) = option {
        res += x;
    }

    println!("{res}");
}
