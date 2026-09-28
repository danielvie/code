pub const GREETING: &str = "Hello";
pub const DEFAULT_NAME: &str = "Rust";

pub fn hello(name: &str) -> String {
    format!("{GREETING}, {name}!")
}

#[cfg(test)]
mod tests {
    use super::hello;

    #[test]
    fn greets_by_name() {
        assert_eq!(hello("Ferris"), "Hello, Ferris!");
    }
}
