fn main() {
    
    // ----------------------
    // vector
    let mut v1: Vec<i32> = Vec::new();
    let v2: Vec<i32> = vec![1, 2, 3];
    
    v1.push(1);

    
    println!("{:?}", v1);
    println!("{:?}", v2);
    
    // ----------------------
    // reading elements from vectors
    let v = vec![1, 2, 3, 4, 5]; 
    let third = &v[2];

    // v[2] = 8;
    
    println!("the third element is {}", third);

    let third = v.get(2);
    if let Some(v) = third {
        println!("third: {}", v);
    }

    println!("third: {}", third.unwrap());

    match third {
        Some(c) => {
            println!("kkk: {}", c);
        },
        None => {
            println!("nada a declarar");
        }
    }
    
    
    // ----------------------
    // rust provides two ways to reference an element

    let mut v = vec![1, 2, 3, 4, 5];

    v.push(88);
    
    let first = &v[0];
    println!("the first element is: {first}");
    
    let first = v.get(0);
    if let Some(v) = first {
        println!("the first element is: {v}");
    }

    // ----------------------
    // iterating over the value in a Vector
    // (see `py_iterator`)
    
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    // you can also 
    
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 10;
    }
    println!("{:?}", v);
    

}
