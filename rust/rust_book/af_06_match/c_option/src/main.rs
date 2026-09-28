// option<T>
// 
fn main()  {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    
    
    
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    
    println!("value: {:?}", none);
    
    
    // let...else
    
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("max is configured to be: {max}"),
        None => (),
    }

    if let Some(max) = config_max {
        println!("max is configured to be: {max}");
    }
    
    
    // ------------------------------------
    
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
    }
        
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }
    
    let coin = Coin::Quarter(UsState::Alaska);
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }

}
