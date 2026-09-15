# Ticket reservation with Quint and Rust model-based testing

This is the Quint version of the [TLA+ ticket exercise](../ticket-reservation/README.md). It tests the **same Rust implementation**, imported as a local Cargo dependency from `../ticket-reservation`. That sibling folder must remain present.

The model, test driver, Taskfile, dependencies, caches, and generated output live here. No global Quint installation is needed.

## Run it

Requirements:

- [Task](https://taskfile.dev/).
- Node.js 18+ and npm. Tested with Node.js 25.6.
- Rust with Cargo, rustfmt, and clippy. Tested with Rust 1.92.
- Java 17+ only for `task verify`.
- Internet access for the initial dependency downloads.

From this folder:

```sh
task                     # List commands
task check               # Install dependencies and run the normal checks
task test                # Generate traces and replay them against Rust
task run                 # Run a short Rust demo
```

Other commands:

| Command | What runs |
| --- | --- |
| `task setup` | Locked npm and Cargo dependencies, plus the native Rust evaluator |
| `task typecheck` | Quint parser, type checker, and effect checker |
| `task model:test` | Five named scenarios inside the model |
| `task simulate` | Invariant checks over 1,000 sampled traces, up to 30 steps each |
| `task verify` | Apalache model checking through 10 steps |
| `task trace` | Export one sampled execution to `artifacts/trace_0.itf.json` |
| `task test:mutation` | Show the state mismatch caused by a deliberately faulty adapter |
| `task build` | Build the Rust demo |
| `task fmt` | Format this project's Rust files |
| `task lint` | Check formatting and Clippy |

`task check` runs typechecking, model scenarios, simulation, linting, and Rust MBT. `task verify` is separate so ordinary tests do not require Java. Both were run successfully during implementation.

Quint is pinned to `0.32.0`, and Quint Connect to `0.1.2`. Both dependency lockfiles are included with the exercise. Tasks use `scripts/with-env.sh` to put the local Quint executable on PATH and keep caches and temporary files inside this folder.

## Read the model

Start with [`model/ticket.qnt`](model/ticket.qnt).

```text
Available → Reserved(customer) → Sold(customer)
    ↑                |
    +----------------+
       cancel or expire
```

The ticket variables are `status`, `owner`, and `buyer`. As in the TLA+ version, the sold state retains the reservation owner so the invariant can compare that owner with the buyer.

`init` establishes the initial values. Each action describes one atomic transition. `step` chooses a customer and then a command nondeterministically.

```quint
action confirm(customer: str): bool =
  if (status == "reserved" and customer == owner) all {
    status' = "sold",
    owner' = owner,
    buyer' = customer,
    accepted' = true,
    lastCommand' = Confirm({ customer: customer }),
  } else reject(Confirm({ customer: customer }))
```

A prime means the variable's value in the next state. `all` requires every condition to hold together; these are not sequential assignments.

`inv` combines type/domain checks, state consistency, and the rule that only the current reservation owner can buy the ticket.

### Why model rejected commands?

A model that only permits successful transitions cannot generate an invalid request. It would never ask Bob to confirm Alice's reservation.

Here, a rejected request is a valid model step with `accepted = false`. It leaves the ticket variables unchanged. The driver compares that outcome with Rust's `Result`, so an unexpected success fails the test even if the ticket state happens not to change.

The model still has the same five possible ticket states. Its complete state space is larger because it also records command outcomes and trace metadata.

### Why record `lastCommand`?

It records the command name and arguments needed for replay. It is a tagged `Command` value, such as `Confirm({ customer: "bob" })`.

In the pinned Quint version, `quint run --mbt` exports automatic action metadata, but named `quint test` traces did not include it in our checks. We use Quint Connect's supported custom `nondet` path to read `lastCommand` in both cases. That also lets named tests call `reserve("alice")` directly.

`lastCommand` is replay metadata, not part of the application's business-state comparison. The model never uses it to decide whether a request succeeds.

## How the model-based tests work

Read [`tests/mbt.rs`](tests/mbt.rs).

```text
Quint model
    ↓ generate traces with expected states
Quint Connect
    ↓ decode lastCommand
TicketDriver::step
    ↓ call Ticket::apply in the existing Rust library
Snapshot::from_driver
    ↓ compare actual status, owner, buyer, and accepted with the model
Pass or a state diff
```

The driver resets the Rust ticket at the start of every trace. Its snapshot reads the actual Rust state and the actual command result. It does not copy expected state from the trace.

There are seven Rust tests:

- One generated test replays 1,000 traces, each up to 30 commands.
- Five tests replay the named Quint scenarios: unauthorized confirmation, expiry first, confirmation first, cancellation, and same-customer reservation reuse.
- One fault-detection test requires the model-based checker to reject an intentionally faulty adapter.

The named cases ensure important scenarios execute even when random sampling misses them. The generated test explores additional command sequences. Tests run serially for readable logs.

### See a real mismatch

```sh
task test:mutation
```

This test-only adapter rewrites every confirmation to act as the reservation owner. It simulates an authorization bug without modifying the sibling Rust library.

The model generates:

```text
Reserve(alice)
Confirm(bob)
```

Expected versus actual:

| Field | Model | Faulty adapter |
| --- | --- | --- |
| `status` | reserved | sold |
| `buyer` | nobody | alice |
| `accepted` | false | true |

Quint Connect prints a mismatch and `[FAIL] deliberately_faulty_adapter`. The enclosing Rust test passes because detecting that exact state mismatch is the expected outcome. A missing executable or a trace-generation error would fail the test instead.

## Reproduce or vary the traces

Tasks default to seed 42. To sample another set:

```sh
QUINT_SEED=7 task test
QUINT_SEED=7 task simulate
QUINT_SEED=7 task trace
```

For step-by-step replay output:

```sh
QUINT_VERBOSE=1 task test
```

To run one scenario with the local environment:

```sh
bash scripts/with-env.sh cargo test --locked --test mbt \
  unauthorized_confirmation_is_rejected -- --nocapture
```

The deliberate fault test always uses its fixed seed 42. Its scenario contains no random choices.

## What was checked

During implementation:

- The five Quint scenarios passed.
- Invariant simulation passed with 1,000 samples at seed 42.
- All seven Rust tests passed with seeds 42 and 7. Each run replayed 1,000 generated traces plus the named scenarios and expected-failure case.
- Apalache found no invariant violation through 10 steps.
- Rust formatting, Clippy, the demo, and trace export passed.

The initial validation logs are in the ignored `artifacts/` directory. Apalache writes its own diagnostics under the ignored `_apalache-out/` directory.

These checks are not an unbounded proof of implementation correctness. MBT samples traces; the Apalache task checks a bounded depth. The existing TLA+ exercise's complete finite graph comparison remains useful and is not replaced.

## Assumptions and next exercise

Transitions are atomic. Customer identity is trusted. Expiry is an explicit event, not a timer. There is no payment processing, database, network, crash recovery, or liveness guarantee.

There are no reservation IDs. An old confirmation from Alice can confirm a newer reservation by Alice. An old expiry event can also release a newer reservation. `sameCustomerReuseTest` documents the first limitation rather than claiming to prevent it.

The next extension is to model reservation IDs, generate stale requests, and require Rust to reject them.

References: [Quint language basics](https://quint-lang.org/docs/language-basics), [model-based testing](https://quint-lang.org/docs/model-based-testing), [Quint Connect](https://github.com/informalsystems/quint-connect), and [Rust API documentation](https://docs.rs/quint-connect).
