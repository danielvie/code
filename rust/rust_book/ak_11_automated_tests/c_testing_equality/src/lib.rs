pub fn add_two(a: u64) -> u64 {
    // a + 3 // bug
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
        // assert_ne!(result, 4);
    }

}