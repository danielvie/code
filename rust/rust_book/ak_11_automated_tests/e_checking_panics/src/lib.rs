use std::panic;

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("guess value must be between 1 and 100, got {value}")
        }
        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[should_panic]
    
    // #[test]
    // fn greeting_contains_name() {
    //     Guess::new(200);
    // }
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    #[test]
    #[should_panic(expected = "guess value must be between 1 and 100")]
    fn greeting_contains_name() {
        Guess::new(200);
    }

}