module ticket {
  pure val Customers = Set("alice", "bob")
  pure val Nobody = "nobody"

  // Explicit trace metadata works for both quint run and quint test.
  type Command = Init
    | Reserve({ customer: str })
    | Confirm({ customer: str })
    | Cancel({ customer: str })
    | Expire
    | SoldIdle

  var status: str
  var owner: str
  var buyer: str
  // Observable outcome of the last command. Init uses true by convention.
  var accepted: bool
  var lastCommand: Command

  action init = all {
    status' = "available",
    owner' = Nobody,
    buyer' = Nobody,
    accepted' = true,
    lastCommand' = Init,
  }

  // Rejection is an observable outcome, not a disabled model transition.
  // This lets generated traces exercise invalid API calls too.
  action reject(command: Command): bool = all {
    status' = status,
    owner' = owner,
    buyer' = buyer,
    accepted' = false,
    lastCommand' = command,
  }

  action reserve(customer: str): bool =
    if (status == "available") all {
      status' = "reserved",
      owner' = customer,
      buyer' = buyer,
      accepted' = true,
      lastCommand' = Reserve({ customer: customer }),
    } else reject(Reserve({ customer: customer }))

  action confirm(customer: str): bool =
    if (status == "reserved" and customer == owner) all {
      status' = "sold",
      owner' = owner,
      buyer' = customer,
      accepted' = true,
      lastCommand' = Confirm({ customer: customer }),
    } else reject(Confirm({ customer: customer }))

  action cancel(customer: str): bool =
    if (status == "reserved" and customer == owner) all {
      status' = "available",
      owner' = Nobody,
      buyer' = buyer,
      accepted' = true,
      lastCommand' = Cancel({ customer: customer }),
    } else reject(Cancel({ customer: customer }))

  // Expiry is an explicit event, not a clock or a reservation-specific timer.
  action expire =
    if (status == "reserved") all {
      status' = "available",
      owner' = Nobody,
      buyer' = buyer,
      accepted' = true,
      lastCommand' = Expire,
    } else reject(Expire)

  // Included to match the original Rust API's terminal self-loop.
  action soldIdle =
    if (status == "sold") all {
      status' = status,
      owner' = owner,
      buyer' = buyer,
      accepted' = true,
      lastCommand' = SoldIdle,
    } else reject(SoldIdle)

  action step = {
    nondet customer = Customers.oneOf()
    any {
      reserve(customer),
      confirm(customer),
      cancel(customer),
      expire,
      soldIdle,
    }
  }

  val typeOK = and {
    Set("available", "reserved", "sold").contains(status),
    Customers.union(Set(Nobody)).contains(owner),
    Customers.union(Set(Nobody)).contains(buyer),
  }

  val stateConsistency = and {
    (status == "available") implies (owner == Nobody and buyer == Nobody),
    (status == "reserved") implies (Customers.contains(owner) and buyer == Nobody),
    (status == "sold") implies (Customers.contains(owner) and Customers.contains(buyer)),
  }

  val onlyOwnerCanBuy = (status == "sold") implies (buyer == owner)
  val inv = typeOK and stateConsistency and onlyOwnerCanBuy

  run unauthorizedConfirmationTest = init
    .then(reserve("alice"))
    .expect(accepted and owner == "alice")
    .then(confirm("bob"))
    .expect(not(accepted) and status == "reserved" and owner == "alice" and buyer == Nobody)

  run expiryFirstTest = init
    .then(reserve("alice"))
    .then(expire)
    .expect(accepted and status == "available")
    .then(confirm("alice"))
    .expect(not(accepted) and status == "available")
    .then(reserve("bob"))
    .then(confirm("alice"))
    .expect(not(accepted) and owner == "bob")
    .then(confirm("bob"))
    .expect(accepted and status == "sold" and buyer == "bob")

  run confirmationFirstTest = init
    .then(reserve("alice"))
    .then(confirm("alice"))
    .expect(accepted and status == "sold")
    .then(expire)
    .expect(not(accepted) and status == "sold" and buyer == "alice")
    .then(reserve("bob"))
    .expect(not(accepted) and status == "sold")

  run cancellationTest = init
    .then(reserve("alice"))
    .then(cancel("bob"))
    .expect(not(accepted) and owner == "alice")
    .then(cancel("alice"))
    .expect(accepted and status == "available")
    .then(reserve("bob"))
    .expect(accepted and owner == "bob")

  // Document the same limitation as the TLA+ exercise: no reservation IDs.
  run sameCustomerReuseTest = init
    .then(reserve("alice"))
    .then(expire)
    .then(reserve("alice"))
    .then(confirm("alice"))
    .expect(accepted and status == "sold")
}
