pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}

pub fn sub(a: i32, b: i32) -> i32 {
    a - b
}

#[cfg(test)]
mod tests {
    use super::{sub, sum};

    #[test]
    fn calculates_sum_and_subtraction() {
        assert_eq!(sum(4, 8), 12);
        assert_eq!(sub(8, 4), 4);
    }
}
