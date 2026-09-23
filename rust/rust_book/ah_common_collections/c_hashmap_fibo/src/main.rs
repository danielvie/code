use std::{collections::HashMap, time::Instant};

fn fib(n: i32, cache: &mut HashMap<i32, i128>) -> i128 {
    if let Some(&value) = cache.get(&n) {
        return value;
    }
    
    let value = match n {
        0 => 0,
        1 => 1,
        _ => fib(n-1, cache)+fib(n-2, cache),
    };
    
    cache.insert(n, value);
    value
}

fn main() {
    let mut cache = HashMap::new();

    for n in 1..100 {
        let start = Instant::now();
        let result = fib(n, &mut cache);
        let duration = start.elapsed();
        println!("fib({n:3}): {result:5} ... (duration: {:?})", duration);
    }
}