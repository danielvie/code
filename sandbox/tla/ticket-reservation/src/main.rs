use std::io::{self, BufRead, Write};
use ticket_reservation::{Action, Customer, Ticket};

fn step(ticket: &mut Ticket, action: Action) {
    let outcome = if ticket.apply(action).is_ok() {
        "accepted"
    } else {
        "rejected"
    };
    println!("{action}: {outcome} -> {:?}", ticket.state());
}

fn demo() {
    use Action::*;
    use Customer::*;

    println!("The counterexample, now rejected by Rust:");
    let mut ticket = Ticket::new();
    step(&mut ticket, Reserve(Alice));
    step(&mut ticket, Confirm(Bob));
    step(&mut ticket, Confirm(Alice));
    step(&mut ticket, Expire);

    println!("\nExpiry wins the race; another customer reserves:");
    let mut ticket = Ticket::new();
    step(&mut ticket, Reserve(Alice));
    step(&mut ticket, Expire);
    step(&mut ticket, Reserve(Bob));
    step(&mut ticket, Confirm(Alice));
    step(&mut ticket, Confirm(Bob));
}

fn parse_action(words: &[&str]) -> Result<Action, &'static str> {
    match words {
        ["expire"] => Ok(Action::Expire),
        ["idle"] => Ok(Action::SoldIdle),
        [command @ ("reserve" | "confirm" | "cancel"), name] => {
            let customer = match *name {
                "alice" => Customer::Alice,
                "bob" => Customer::Bob,
                _ => return Err("customer must be alice or bob"),
            };
            Ok(match *command {
                "reserve" => Action::Reserve(customer),
                "confirm" => Action::Confirm(customer),
                "cancel" => Action::Cancel(customer),
                _ => unreachable!(),
            })
        }
        _ => Err("use reserve/confirm/cancel <alice|bob>, expire, idle, show, or quit"),
    }
}

fn interactive() -> io::Result<()> {
    println!("One ticket. Commands: reserve/confirm/cancel <alice|bob>, expire, idle, show, quit");
    let mut ticket = Ticket::new();
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    loop {
        print!("> ");
        io::stdout().flush()?;
        let Some(line) = lines.next() else { break };
        let line = line?;
        let words: Vec<_> = line.split_whitespace().collect();
        match words.as_slice() {
            [] => {}
            ["quit"] => break,
            ["show"] => println!("{:?}", ticket.state()),
            words => match parse_action(words) {
                Ok(action) => step(&mut ticket, action),
                Err(message) => println!("{message}"),
            },
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    match args.as_slice() {
        [] => demo(),
        [arg] if arg == "--interactive" => interactive()?,
        _ => {
            eprintln!("usage: ticket-reservation [--interactive]");
            std::process::exit(2);
        }
    }
    Ok(())
}
