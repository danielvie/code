use quint_connect::{Driver, Result, State as ModelState, Step, quint_run, quint_test, switch};
use serde::Deserialize;
use ticket_reservation::{Action, Customer, State, Ticket};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Actor {
    Alice,
    Bob,
}

impl From<Actor> for Customer {
    fn from(actor: Actor) -> Self {
        match actor {
            Actor::Alice => Self::Alice,
            Actor::Bob => Self::Bob,
        }
    }
}

// These fields match the model variables, not the implementation's layout.
#[derive(Debug, Deserialize, Eq, PartialEq)]
struct Snapshot {
    status: String,
    owner: String,
    buyer: String,
    accepted: bool,
}

impl ModelState<TicketDriver> for Snapshot {
    fn from_driver(driver: &TicketDriver) -> Result<Self> {
        let (status, owner, buyer) = match driver.ticket.state() {
            State::Available => ("available", "nobody".into(), "nobody".into()),
            State::Reserved { owner } => ("reserved", owner.to_string(), "nobody".into()),
            State::Sold { owner, buyer } => ("sold", owner.to_string(), buyer.to_string()),
        };
        Ok(Self {
            status: status.into(),
            owner,
            buyer,
            accepted: driver.accepted,
        })
    }
}

struct TicketDriver {
    ticket: Ticket,
    accepted: bool,
    // Test-only fault: impersonate the owner when handling confirmation.
    // The sibling Rust library itself is never changed.
    impersonate_owner: bool,
}

impl Default for TicketDriver {
    fn default() -> Self {
        Self {
            ticket: Ticket::new(),
            accepted: true,
            impersonate_owner: false,
        }
    }
}

impl TicketDriver {
    fn apply(&mut self, mut action: Action) {
        if self.impersonate_owner
            && let (State::Reserved { owner }, Action::Confirm(_)) = (self.ticket.state(), action)
        {
            action = Action::Confirm(owner);
        }
        self.accepted = self.ticket.apply(action).is_ok();
    }
}

impl Driver for TicketDriver {
    type State = Snapshot;

    fn config() -> quint_connect::Config {
        // Quint 0.32's named-test traces omit automatic MBT metadata.
        // Read the explicit Command sum type for both generation modes.
        quint_connect::Config {
            nondet: &["lastCommand"],
            ..Default::default()
        }
    }

    fn step(&mut self, step: &Step) -> Result {
        switch!(step {
            Init => {
                // Each trace starts fresh. Preserve the fault setting.
                self.ticket = Ticket::new();
                self.accepted = true;
            },
            Reserve(customer: Actor) => self.apply(Action::Reserve(customer.into())),
            Confirm(customer: Actor) => self.apply(Action::Confirm(customer.into())),
            Cancel(customer: Actor) => self.apply(Action::Cancel(customer.into())),
            Expire => self.apply(Action::Expire),
            SoldIdle => self.apply(Action::SoldIdle),
        })
    }
}

#[quint_run(spec = "model/ticket.qnt", max_samples = 1000, max_steps = 30)]
fn generated_sequences_match_rust() -> impl Driver {
    TicketDriver::default()
}

#[quint_test(
    spec = "model/ticket.qnt",
    test = "unauthorizedConfirmationTest",
    max_samples = 1
)]
fn unauthorized_confirmation_is_rejected() -> impl Driver {
    TicketDriver::default()
}

#[quint_test(spec = "model/ticket.qnt", test = "expiryFirstTest", max_samples = 1)]
fn expiry_wins_the_race() -> impl Driver {
    TicketDriver::default()
}

#[quint_test(
    spec = "model/ticket.qnt",
    test = "confirmationFirstTest",
    max_samples = 1
)]
fn confirmation_wins_the_race() -> impl Driver {
    TicketDriver::default()
}

#[quint_test(spec = "model/ticket.qnt", test = "cancellationTest", max_samples = 1)]
fn only_owner_can_cancel() -> impl Driver {
    TicketDriver::default()
}

#[quint_test(
    spec = "model/ticket.qnt",
    test = "sameCustomerReuseTest",
    max_samples = 1
)]
fn reservation_id_limitation_matches_rust() -> impl Driver {
    TicketDriver::default()
}

#[test]
fn faulty_adapter_is_detected() {
    // Run the same model trace against a deliberately faulty integration.
    // Require a state mismatch, not just any failure such as a missing tool.
    let driver = TicketDriver {
        impersonate_owner: true,
        ..TicketDriver::default()
    };
    let config = quint_connect::runner::Config {
        test_name: "deliberately_faulty_adapter".into(),
        gen_config: quint_connect::runner::TestConfig {
            spec: "model/ticket.qnt".into(),
            test: "unauthorizedConfirmationTest".into(),
            main: None,
            max_samples: Some(1),
            seed: "42".into(),
        },
    };
    let error = quint_connect::runner::run_test(driver, config)
        .expect_err("the model-based test must reject the faulty adapter");
    assert_eq!(error.to_string(), "State invariant failed");
}
