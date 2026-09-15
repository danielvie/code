# Ticket reservation, from TLA+ to Rust

Two customers, Alice and Bob, compete for one ticket. We describe the allowed transitions in TLA+, check them with TLC, then implement them in Rust.

Everything for this exercise lives in this folder. Downloads go in `tools/`, TLC output in `artifacts/`, and Rust build output in `target/`. There are no third-party Rust dependencies.

## Run it

Requirements: Java 17+, Python 3.9+, curl, and Rust with edition 2024 support, rustfmt, and clippy.

From this folder, use [Task](https://taskfile.dev/) if installed:

```sh
task                  # List commands
task run              # Run the demo
task interactive      # Open the interactive shell
task check            # Run the complete validation
task model            # Check TLA+ and fixture freshness only
task test             # Run Rust tests using the saved fixture
task build            # Build the app
task fmt              # Format Rust source
task lint             # Check formatting and Clippy
```

`task model:update-fixture` regenerates the fixture after an intentional model change. Task commands keep Cargo's cache and build output inside this folder.

Or run the underlying commands directly:

```sh
# Check the broken model, the fixed model, graph conformance, and Rust tests.
bash scripts/check.sh

# Keep Cargo's cache and build output local for these commands too.
export CARGO_HOME="$PWD/tools/cargo-home"
export CARGO_TARGET_DIR="$PWD/target"

# Show rejection of the counterexample and an expiry race.
cargo run --offline

# Operate the state machine yourself.
cargo run --offline -- --interactive
```

Try these interactive commands:

```text
reserve alice
confirm bob
show
expire
reserve bob
confirm alice
confirm bob
quit
```

The full check intentionally prints an invariant violation for the broken model. That is an expected result, not a failed check. The script requires TLC's invariant-violation exit code and the specific `OnlyOwnerCanBuy` error. It then requires the fixed model to pass.

## 1. Decide what the model means

The rules are:

- Either customer can reserve an available ticket.
- Only the reservation owner can confirm or cancel it.
- An expiry event releases a reserved ticket.
- A sold ticket cannot be reserved, cancelled, expired, or sold again.
- A rejected command changes nothing.

```text
                         confirm(owner)
Available → Reserved(owner) → Sold(owner, buyer)
    ↑              |
    +--------------+
      cancel(owner)
      or expire
```

We retain the reservation owner after a sale so we can compare the buyer with the person who held the reservation.

The model has three variables:

| Variable | Meaning |
| --- | --- |
| `status` | `"available"`, `"reserved"`, or `"sold"` |
| `owner` | Reservation holder, or `nobody` when available |
| `buyer` | Customer who bought the ticket, or `nobody` before sale |

A state is the entire tuple, not just `status`. That gives the fixed model five reachable states:

```text
available, nobody, nobody
reserved,  alice,  nobody
reserved,  bob,    nobody
sold,      alice,  alice
sold,      bob,    bob
```

### Assumptions we are making

Each transition happens atomically. Concurrent commands appear in some order. We are not modeling a separate availability read followed later by a database write. Splitting those steps could introduce bugs this model cannot see.

Customer identity is trustworthy. The ownership guard is an authorization rule, not an authentication system. The CLI lets you act as either customer for experimentation.

Expiry is an explicit event affecting the current reservation. There is no clock, payment provider, network, persistence, or recovery after a crash. The model does not promise that expiry will actually happen.

## 2. Read the TLA+

Open [`model/Ticket.tla`](model/Ticket.tla). `Init` describes the initial state. Each action describes a relation between a current state and a next state.

For example:

```tla
Reserve(c) ==
    /\ status = "available"
    /\ status' = "reserved"
    /\ owner' = c
    /\ UNCHANGED buyer
```

Read it as: this action is allowed when the ticket is available; in the next state it is reserved by `c`, and the buyer has not changed.

These formulas are not sequential assignments. All conjuncts must hold together.

| Syntax | Read it as |
| --- | --- |
| `==` | Define an operator |
| `=` | Equality |
| `status'` | The value of `status` in the next state |
| `/\` | And |
| `\/` | Or |
| `\in` | Is a member of |
| `\E c \in Customers : ...` | There exists a customer for whom this holds |
| `UNCHANGED buyer` | `buyer' = buyer` |
| `P => Q` | If P holds, Q must hold |
| `[]P` | P holds always |

`Next` permits any enabled reserve, confirm, cancel, expiry, or terminal idle action. TLC explores all enabled choices, not a chosen script of commands.

```tla
Spec == Init /\ [][Next]_vars
```

This says to start in `Init`, then always take either a `Next` step or a stuttering step that leaves all variables unchanged. Real implementations can do work that does not change the modeled state, including rejecting an invalid command.

`SoldIdle` explicitly gives sold states a self-loop. TLC's deadlock check asks whether `Next` has a successor. Without that self-loop it would report the intentionally terminal sale as a deadlock, even though the temporal specification permits stuttering.

## 3. State what must never go wrong

The `.cfg` files tell TLC which concrete customers to use and which invariants to check.

An invariant must hold in every reachable state:

- `TypeOK` checks the allowed values of all variables.
- `StateConsistency` checks that available tickets have no owner or buyer, reserved tickets have an owner but no buyer, and sold tickets have both.
- `OnlyOwnerCanBuy` requires `buyer = owner` whenever the ticket is sold.

The last property needs the old owner as evidence. If `Confirm` overwrote both `owner` and `buyer` with the caller, the invariant would no longer detect an unauthorized purchase. A checker can only check the property you actually expressed.

These are safety checks. We are not checking liveness, such as "every reservation eventually ends." Infinite stuttering or a reservation that never expires is allowed. Adding a progress claim requires deciding what scheduling or fairness assumptions are justified.

## 4. Let TLC find a bug

[`model/Buggy.cfg`](model/Buggy.cfg) sets `BuggyConfirm = TRUE`. In the confirmation action:

```tla
/\ (BuggyConfirm \/ c = owner)
```

With the flag true, the ownership guard imposes no restriction. TLC finds:

```text
1. Available
2. Reserve(alice)  → owner = alice
3. Confirm(bob)    → buyer = bob, owner = alice
```

State 3 violates `OnlyOwnerCanBuy`. This is a counterexample: a concrete sequence of allowed transitions that reaches a forbidden state.

The full trace is in `artifacts/buggy.log` after running the check. The error appears after two transitions. TLC used breadth-first exploration to find it.

[`model/Ticket.cfg`](model/Ticket.cfg) sets the flag false, so confirmation requires `c = owner`. The fixed check reports:

```text
Model checking completed. No error has been found.
11 states generated, 5 distinct states found, 0 states left on queue.
```

The generated count includes repeated discoveries. The five distinct states are the reachable state space. Cycles allow arbitrarily long executions, but TLC does not need to revisit every cycle forever to check these invariants.

This is exhaustive finite-state model checking for the configured two customers, subject to TLC's fingerprint-based state storage. It is not a mathematical proof for every possible customer set or every real-world implementation.

## 5. Implement the transitions in Rust

Read [`src/lib.rs`](src/lib.rs), particularly `Ticket::apply`.

| TLA+ | Rust |
| --- | --- |
| `Init` | `Ticket::new()` |
| `status`, `owner`, `buyer` | `State`, projected by `State::model_key()` |
| An action's guard | Match pattern and `if` guard |
| Primed variables | The returned next `State` |
| Disabled action | `Err(Disabled)`, with no mutation |
| `SoldIdle` | `Action::SoldIdle`, with no mutation |

The `Ticket` field is private, so callers cannot install an arbitrary state. `apply` computes the next state before replacing the current one.

The mutable reference prevents safe Rust callers from concurrently mutating the same ticket without synchronization. For shared access, hold a mutex across the entire `apply` call. The threaded test shows this pattern; it does not prove all possible threaded programs correct.

`src/main.rs` provides a demonstration and an interactive shell. The shell is single-threaded, so it lets you choose the order of competing events yourself.

## 6. Check that Rust matches the model

Hand-picked regression tests replay the counterexample and both orderings of confirmation versus expiry. They also cover cancellation and competing reservation threads.

The conformance test goes further:

1. TLC exports the fixed model's states and labeled transitions to `artifacts/states.dot`.
2. `scripts/export_graph.py` converts that output to [`tests/fixtures/tlc-graph.tsv`](tests/fixtures/tlc-graph.tsv).
3. The full check verifies that the committed fixture matches a fresh TLC run.
4. `tests/conformance.rs` explores Rust from `Ticket::new()`, trying all eight actions in every reachable state.
5. It compares the initial state, reachable states, and successful labeled transitions with TLC's graph. Every rejection must leave the state unchanged.

The result is **5 matching states, 10 matching labeled transitions, and 40 command attempts**. Seven Rust tests pass in total.

The plain `cargo test` command uses the saved fixture and does not invoke Java. Run `bash scripts/check.sh` when changing the model so a stale fixture cannot silently pass. After reviewing an intentional model change, regenerate it with:

```sh
python3 scripts/check_model.py --update-fixture
bash scripts/check.sh
```

This is executable conformance evidence for this small deterministic API and its state projection. It is not a machine-checked refinement proof. The fixture converter, projection, and test code can also contain bugs.

## Next exercise: stale requests

Consider this sequence:

```text
Alice reserves
Alice's reservation expires
Alice reserves again
An old confirmation from Alice arrives
```

Our current model accepts the confirmation. It knows the customer, but not which reservation the request belongs to. The corresponding Rust test documents that limitation. An old expiry event could likewise release a newer reservation.

To reject those stale operations, extend the model with reservation identifiers and attach the identifier to confirm, cancel, and expiry events. Decide how to bound identifiers for TLC without assuming that identifier reuse is harmless. Model the failure before changing Rust.

Further reading: [Leslie Lamport's TLA+ video course](https://lamport.azurewebsites.net/video/videos.html) and [Specifying Systems](https://lamport.azurewebsites.net/tla/book.html).
