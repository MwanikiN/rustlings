trait Licensed {
    fn licensing_info(&self) -> String {
        "Default license".to_string() //returns a string
    }
}

struct SomeSoftware;
struct OtherSoftware;

impl Licensed for SomeSoftware {}
impl Licensed for OtherSoftware {}

// TODO: Fix the compiler error by only changing the signature of this function.
fn compare_license_types(software1: impl Licensed, software2: impl Licensed) -> bool {
    software1.licensing_info() == software2.licensing_info()
}

//alternatively using generics
fn compare_license_types_generic<T: Licensed, U: Licensed>(software1: T, software2: U) -> bool {
    software1.licensing_info() == software2.licensing_info()
}

fn main() {
    // You can optionally experiment here.
    assert!(compare_license_types_generic(SomeSoftware, OtherSoftware));
    let software1 = SomeSoftware.licensing_info();
    let software2 = OtherSoftware.licensing_info();
    println!("SomeSoftware licensing info: {software1}");
    println!("OtherSoftware licensing info: {software2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_license_information() {
        assert!(compare_license_types(SomeSoftware, OtherSoftware));
    }

    #[test]
    fn compare_license_information_backwards() {
        assert!(compare_license_types(OtherSoftware, SomeSoftware));
    }
}
