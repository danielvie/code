fn main()  {
    
    // a match that only cares about executing code when value
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        None => (),
    }
    

    // more compact control flow
    println!("estamos aqui");
    println!("============\n");
    
    // let config_max = Some(3u8);
    let config_max = None::<u8>;
    if let Some(m) = config_max {
        println!("The maximum is configured to be {m}");
    }

    println!("\n============");
    println!("ateh estamos aqui");
    
    // control flow with `else`
    let number = None::<u8>;
    if let Some(value) = number {
        println!("The number is {value}");
    } else {
        println!("There is no number");
    }
    
    // staying in the "happy path"
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
    
    impl UsState {
        fn existed_in(&self, year: u16) -> bool {
            match self {
                UsState::Alabama => {
                    year >= 1819
                },
                UsState::Alaska => year >= 1959,
                // -- snip --
            }
        }
    }
    
    fn describe_state_quarter(coin: Coin) -> Option<String> {
        let Coin::Quarter(st) = coin else {
            return None;
        };
    
        if st.existed_in(1900) {
            return Some(format!("{st:?} is pretty old, for America!"));
        } else {
            return Some(format!("{st:?} is relatively new."));
        }
    }

    let res = describe_state_quarter(Coin::Dime);
    println!("{:?}", res);
    
    let res = describe_state_quarter(Coin::Quarter(UsState::Alabama));
    println!("{:?}", res);
    
    if let Some(res) = describe_state_quarter(Coin::Quarter(UsState::Alabama)) {
        println!("{:?}", res);
    }
    
    
    
    
    
    
    
    
    
    
    let res = describe_state_quarter(Coin::Quarter(UsState::Alaska)).unwrap_or(String::from("bla"));
    println!("{:?}", res);
    
    let res = describe_state_quarter(Coin::Dime).unwrap_or(String::from("bla"));
    println!("{:?}", res);
    
}
