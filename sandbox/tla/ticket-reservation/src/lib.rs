use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Customer {
    Alice,
    Bob,
}

impl fmt::Display for Customer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Alice => "alice",
            Self::Bob => "bob",
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum State {
    Available,
    Reserved { owner: Customer },
    // Retain the reservation owner to match the TLA+ invariant's evidence.
    Sold { owner: Customer, buyer: Customer },
}

impl State {
    /// Projection to the TLA+ variables: status, owner, buyer.
    pub fn model_key(self) -> String {
        match self {
            Self::Available => "available,nobody,nobody".into(),
            Self::Reserved { owner } => format!("reserved,{owner},nobody"),
            Self::Sold { owner, buyer } => format!("sold,{owner},{buyer}"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
    Reserve(Customer),
    Confirm(Customer),
    Cancel(Customer),
    Expire,
    /// The model's terminal self-loop. No business operation is performed.
    SoldIdle,
}

impl Action {
    pub const ALL: [Self; 8] = [
        Self::Reserve(Customer::Alice),
        Self::Reserve(Customer::Bob),
        Self::Confirm(Customer::Alice),
        Self::Confirm(Customer::Bob),
        Self::Cancel(Customer::Alice),
        Self::Cancel(Customer::Bob),
        Self::Expire,
        Self::SoldIdle,
    ];
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Reserve(c) => write!(f, "Reserve({c})"),
            Self::Confirm(c) => write!(f, "Confirm({c})"),
            Self::Cancel(c) => write!(f, "Cancel({c})"),
            Self::Expire => f.write_str("Expire"),
            Self::SoldIdle => f.write_str("SoldIdle"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Disabled;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Ticket {
    // Callers cannot install an arbitrary State into a Ticket.
    state: State,
}

impl Default for Ticket {
    fn default() -> Self {
        Self::new()
    }
}

impl Ticket {
    pub const fn new() -> Self {
        Self {
            state: State::Available,
        }
    }

    pub fn state(&self) -> State {
        self.state
    }

    /// One atomic model step. Rejected commands leave the state unchanged.
    ///
    /// A shared Ticket must be locked for this entire call. Customer identity
    /// is assumed trustworthy here; this library does not authenticate callers.
    pub fn apply(&mut self, action: Action) -> Result<(), Disabled> {
        let next = match (self.state, action) {
            (State::Available, Action::Reserve(owner)) => State::Reserved { owner },
            (State::Reserved { owner }, Action::Confirm(customer)) if customer == owner => {
                State::Sold {
                    owner,
                    buyer: customer,
                }
            }
            (State::Reserved { owner }, Action::Cancel(customer)) if customer == owner => {
                State::Available
            }
            (State::Reserved { .. }, Action::Expire) => State::Available,
            (State::Sold { .. }, Action::SoldIdle) => self.state,
            _ => return Err(Disabled),
        };
        self.state = next;
        Ok(())
    }
}
