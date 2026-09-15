use std::sync::{Arc, Barrier, Mutex};
use std::thread;
use ticket_reservation::{Action::*, Customer::*, Disabled, State, Ticket};

#[test]
fn tlc_counterexample_is_rejected() {
    let mut ticket = Ticket::new();
    ticket.apply(Reserve(Alice)).unwrap();
    let before = ticket;
    assert_eq!(ticket.apply(Confirm(Bob)), Err(Disabled));
    assert_eq!(ticket, before);
    ticket.apply(Confirm(Alice)).unwrap();
    assert_eq!(
        ticket.state(),
        State::Sold {
            owner: Alice,
            buyer: Alice
        }
    );
}

#[test]
fn confirmation_wins_expiry_race() {
    let mut ticket = Ticket::new();
    ticket.apply(Reserve(Alice)).unwrap();
    ticket.apply(Confirm(Alice)).unwrap();
    let sold = ticket;
    for action in [
        Expire,
        Cancel(Alice),
        Reserve(Bob),
        Confirm(Alice),
        Confirm(Bob),
    ] {
        assert_eq!(ticket.apply(action), Err(Disabled));
        assert_eq!(ticket, sold);
    }
}

#[test]
fn expiry_wins_confirmation_race() {
    let mut ticket = Ticket::new();
    ticket.apply(Reserve(Alice)).unwrap();
    ticket.apply(Expire).unwrap();
    assert_eq!(ticket.apply(Confirm(Alice)), Err(Disabled));
    ticket.apply(Reserve(Bob)).unwrap();
    assert_eq!(ticket.apply(Confirm(Alice)), Err(Disabled));
    ticket.apply(Confirm(Bob)).unwrap();
    assert_eq!(
        ticket.state(),
        State::Sold {
            owner: Bob,
            buyer: Bob
        }
    );
}

#[test]
fn only_owner_can_cancel() {
    let mut ticket = Ticket::new();
    ticket.apply(Reserve(Alice)).unwrap();
    assert_eq!(ticket.apply(Cancel(Bob)), Err(Disabled));
    ticket.apply(Cancel(Alice)).unwrap();
    assert_eq!(ticket.state(), State::Available);
}

#[test]
fn same_customer_can_confirm_a_new_reservation_without_a_token() {
    // A documented limitation, not a claim of stale-request protection.
    // The model cannot distinguish old and new requests from the same customer.
    let mut ticket = Ticket::new();
    ticket.apply(Reserve(Alice)).unwrap();
    ticket.apply(Expire).unwrap();
    ticket.apply(Reserve(Alice)).unwrap();
    assert_eq!(ticket.apply(Confirm(Alice)), Ok(()));
}

#[test]
fn mutex_keeps_the_entire_reservation_transition_atomic() {
    let ticket = Arc::new(Mutex::new(Ticket::new()));
    let barrier = Arc::new(Barrier::new(2));
    let handles: Vec<_> = [Alice, Bob]
        .into_iter()
        .map(|customer| {
            let ticket = Arc::clone(&ticket);
            let barrier = Arc::clone(&barrier);
            thread::spawn(move || {
                barrier.wait();
                (customer, ticket.lock().unwrap().apply(Reserve(customer)))
            })
        })
        .collect();
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    let winners: Vec<_> = results
        .iter()
        .filter(|(_, result)| result.is_ok())
        .collect();
    assert_eq!(winners.len(), 1);
    assert_eq!(
        ticket.lock().unwrap().state(),
        State::Reserved {
            owner: winners[0].0
        }
    );
}
