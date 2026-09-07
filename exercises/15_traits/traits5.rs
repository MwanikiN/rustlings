trait SomeTrait {
    fn some_function(&self) -> bool {
        true
    }
}

trait OtherTrait {
    fn other_function(&self) -> bool {
        true
    }
}

struct SomeStruct; //Unit struct. Recall they have 0 bytes.
impl SomeTrait for SomeStruct {}
impl OtherTrait for SomeStruct {}

struct OtherStruct; //Unit struct as well. Check documentation on implementation and use cases.
impl SomeTrait for OtherStruct {}
impl OtherTrait for OtherStruct {}

// TODO: Fix the compiler error by only changing the signature of this function.
fn some_func(item: impl SomeTrait + OtherTrait) -> bool { //both traits are required to be implemented for the function to work. This is called a trait bound. The function will only accept items that implement both traits.
    item.some_function() && item.other_function()
}

//alternatively lets use generics
fn some_func_generic<T: SomeTrait + OtherTrait>(item: T) -> bool {
    item.some_function() && item.other_function()
}

fn main() {
    // You can optionally experiment here.
    //implementing the above traits to test them out 
    let some_struct = SomeStruct;
    let other_struct = OtherStruct;
    println!("SomeStruct: {}", some_func_generic(some_struct));
    println!("OtherStruct: {}", some_func(other_struct));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_some_func() {
        assert!(some_func(SomeStruct));
        assert!(some_func(OtherStruct));
    }
}
