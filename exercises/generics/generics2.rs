// generics2.rs
//
// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.
//
// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a
// hint.



struct Wrapper<T> {
    value: T,
}

impl Wrapper<u32> {
    pub fn new_u32(value: u32) -> Self {
        Wrapper { value }
    }
}

impl Wrapper<String>{
    pub fn new_str(value: &str) ->Self{
        Wrapper { value: value.to_string() }
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new_u32(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new_str("Foo").value, "Foo");
    }
}
