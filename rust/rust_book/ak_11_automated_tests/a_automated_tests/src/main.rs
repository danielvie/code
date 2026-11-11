pub fn add(l: u64, r: u64) -> u64 {
    l + r
}

fn main() {
    
    // a test in Rust is a function that is annotated with the `test` attibute
    // attributes are metadata about pieces of Rust code
    // example: `derive`
    
    //     struct A {
    //         a: i32,
    //         b: i32,
    //     }
    // 
    //     let aa = A {
    //         a: 12,
    //         b: 12,
    //     };
    //     
    //     println!("Hello, world! {aa:?}");


    // to change a function into test function, add #[test]
    // to run a test: `cargo test`
    
    
}

// boilerplate
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bla() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn another() {
        panic!("make this fail");
    }
}