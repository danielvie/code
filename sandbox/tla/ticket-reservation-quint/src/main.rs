use ticket_reservation::{Action::*, Customer::*, Ticket};

fn main() {
    let mut ticket = Ticket::new();
    for action in [
        Reserve(Alice),
        Confirm(Bob),
        Expire,
        Reserve(Bob),
        Confirm(Alice),
        Confirm(Bob),
        Expire,
    ] {
        let result = ticket.apply(action);
        println!("{action}: {result:?} -> {:?}", ticket.state());
    }
}
