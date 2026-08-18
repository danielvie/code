fn main() {
    
    // let mut cont: i32 = 0;
    // loop {
    //     cont += 1;
    //     println!("{cont}");
    // }
    
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    
    println!("The result is {result}");
    
}
