use std::collections::HashMap;

fn main() {
    
    // hashmaps: 
    // the type is HashMap<K, V>
    // stores a mapping of keys of type K to values of type V
    // 
    // are useful when you want to look up data not by using an index
    
    let mut scores = HashMap::new();

    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 50);
    
    println!("blue: {}", scores["blue"]);
    // println!("yellow: {}", scores.get("blue").);
    
    let name  = String::from("favorite color");
    let value = String::from("blue");
    
    let mut map = HashMap::new();
    map.insert(name, value);
    // println!("name: {}", map["favorite color"]);
    
    // WRITE A VALUE
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{scores:?}");
    

    // adding a key and value if a key inst present
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    // scores.entry(String::from("kjkjkj")).or_insert(50);
    // println!("debug:: {:?}", scores.entry(String::from("Yellow")));
    // println!("debug:: {:?}", scores.entry(String::from("kjkj")));
    
    println!("\n=============================");
    println!("{scores:?}");
    
    // another common use is to loop up a key's value and then update
    
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("\n=============================");
    println!("{map:?}");
    
}
