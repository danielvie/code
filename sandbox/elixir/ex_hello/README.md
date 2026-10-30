# Hello Elixir state machine

Run from this directory with Elixir and Erlang on your PATH:

```sh
task run
task test
task version
```

Without Task, use `elixir hello.exs` or `elixir test/questionnaire_test.exs`.

The questionnaire asks for your name and Elixir experience, then asks you to
confirm. At confirmation, enter `name` or `experience` to edit an answer.
Changing your name also asks the experience question again. Enter `restart`
or `quit` at any question. These two words are reserved commands, not names.

```text
ask_name -> ask_experience -> confirm -> done
    ^              ^            |
    |              +------------+ experience
    +---------------------------+ name

Any active state -- restart --> ask_name, clearing answers
Any active state -- quit -----> quit
```

## Declarative rules

`questionnaire.ex` describes the machine using a small local Elixir macro DSL:

```elixir
state :ask_name do
  prompt "What is your name?"
  fallback "Please enter a non-empty name."

  on :answer do
    guard fn input, _tracks -> input != "" end
    update fn input, tracks -> %{tracks | name: input} end
    reply "Name saved."
    goto :ask_experience
  end
end
```

- `initial` selects the starting state.
- `track` declares an answer field and its default value.
- `state` groups a prompt, fallback message, and event rules.
- `on` selects an event. Multiple rules can handle the same event.
- `guard` receives the input and current tracks. Omit it to accept the event unconditionally.
- `update` returns the complete new tracks map. Omit it to preserve the tracks.
- `reply` is a string or a function receiving the updated tracks.
- `goto` selects the next state, which may be the current state.
- `global` declares rules shared by active states.
- `terminal` declares an end state that ignores all further events.

Global rules run before local rules. Within either group, the first rule whose
event matches and whose guard passes wins. If no rule passes, the machine stays
unchanged and returns the state's `fallback` message. A `prompt` can also be a
function receiving the current tracks.

Here, `guard` is a DSL callback, not an Elixir `when` guard. It can call ordinary
functions such as `String.downcase/1`.

## Files

- `hello.exs` loads and starts the questionnaire.
- `questionnaire.ex` declares the rules and translates console answers into events.
- `state_machine.ex` defines `defmachine` and the pure transition engine.
- `console.ex` reads input and prints prompts, messages, and states.
- `test/questionnaire_test.exs` tests the questionnaire and DSL engine.

`defmachine` generates `new/0`, `state_spec/1`, and `global_rules/0`. The engine
uses those definitions to return `{next_machine, message}`. It does not mutate
the previous machine or perform console I/O.

This is a teaching example inspired by the screenshot's style, not the Accord
library. It does not implement typed events, bounded domains, formal verification,
persistence, or an OTP process. The console loop owns the current state.
