Code.require_file("../questionnaire.ex", __DIR__)
ExUnit.start()

defmodule Hello.QuestionnaireTest do
  use ExUnit.Case, async: true

  alias Hello.Questionnaire, as: Questionnaire

  defp answer(machine, input) do
    {next, _message} = Questionnaire.transition(machine, input)
    next
  end

  defp confirmation do
    Questionnaire.new()
    |> answer("Ada")
    |> answer("no")
  end

  test "starts by asking for a name" do
    assert Questionnaire.new() == %{state: :ask_name, name: nil, experience: nil}
  end

  test "completes the questionnaire for new and returning users" do
    for experience <- ["yes", "no"] do
      named = answer(Questionnaire.new(), "  Ada  ")
      assert named.state == :ask_experience
      assert named.name == "Ada"

      confirmed = answer(named, String.upcase(experience))
      assert confirmed.state == :confirm
      assert confirmed.experience == experience

      {finished, message} = Questionnaire.transition(confirmed, " YES \n")
      assert finished.state == :done
      assert message =~ "Hello, Ada!"
      assert message =~ if(experience == "yes", do: "Welcome back", else: "first Elixir program")
    end
  end

  test "invalid answers leave the machine unchanged" do
    initial = Questionnaire.new()
    assert answer(initial, " \n") == initial

    named = answer(initial, "Ada")
    assert answer(named, "maybe") == named

    confirmed = answer(named, "no")
    assert answer(confirmed, "maybe") == confirmed
  end

  test "can edit a name before confirming" do
    editing = answer(confirmation(), "name")
    assert editing.state == :ask_name

    updated = editing |> answer("Grace") |> answer("yes")
    assert updated.state == :confirm
    assert updated.name == "Grace"
  end

  test "can edit experience before confirming" do
    editing = answer(confirmation(), "experience")
    assert editing.state == :ask_experience

    updated = answer(editing, "yes")
    assert updated.state == :confirm
    assert updated.name == "Ada"
    assert updated.experience == "yes"
  end

  test "restart clears answers and quit ends each active state" do
    states = [Questionnaire.new(), answer(Questionnaire.new(), "Ada"), confirmation()]

    for machine <- states do
      assert answer(machine, " RESTART \n") == Questionnaire.new()
      assert answer(machine, " QUIT \n").state == :quit
    end
  end

  test "terminal states cannot be changed" do
    for machine <- [answer(confirmation(), "yes"), answer(Questionnaire.new(), "quit")] do
      assert answer(machine, "restart") == machine
      assert Questionnaire.terminal?(machine)
    end
  end

  test "prompts use the current tracks" do
    assert Questionnaire.prompt(Questionnaire.new()) == "What is your name?"
    assert Questionnaire.prompt(answer(Questionnaire.new(), "Ada")) =~ "Hi, Ada!"
    assert Questionnaire.prompt(confirmation()) =~ "Name: Ada, Elixir experience: no."
  end
end

defmodule Hello.CounterExample do
  import Hello.StateMachine, only: [defmachine: 1]

  defmachine do
    initial(:counting)
    track(:count, default: 0)

    state :counting do
      prompt("Increment?")
      fallback("Limit reached or unknown event.")

      on :increment do
        guard(fn amount, tracks -> tracks.count + amount <= 3 end)
        update(fn amount, tracks -> %{tracks | count: tracks.count + amount} end)
        reply(fn tracks -> "Count: #{tracks.count}" end)
        goto(:counting)
      end

      on :finish do
        reply("Finished.")
        goto(:done)
      end
    end

    terminal(:done)
  end
end

defmodule Hello.StateMachineTest do
  use ExUnit.Case, async: true
  alias Hello.StateMachine
  alias Hello.CounterExample, as: Counter

  test "self-transitions update tracks and replies see the updated values" do
    assert {%{state: :counting, count: 2}, "Count: 2"} =
             StateMachine.transition(Counter, Counter.new(), :increment, 2)
  end

  test "guards inspect cumulative data and rejection preserves it" do
    {machine, _} = StateMachine.transition(Counter, Counter.new(), :increment, 2)

    assert {^machine, "Limit reached or unknown event."} =
             StateMachine.transition(Counter, machine, :increment, 2)
  end

  test "unknown events use the fallback without changing state" do
    machine = Counter.new()
    assert {^machine, _} = StateMachine.transition(Counter, machine, :unknown, nil)
  end

  test "omitted guard and update allow a transition that preserves tracks" do
    assert {%{state: :done, count: 0}, "Finished."} =
             StateMachine.transition(Counter, Counter.new(), :finish, nil)
  end

  test "DSL rejects undeclared destinations at compile time" do
    assert_raise ArgumentError, "unknown destination: :missing", fn ->
      Code.compile_string("""
      defmodule Hello.InvalidDestination do
        import Hello.StateMachine, only: [defmachine: 1]
        defmachine do
          initial :start
          state :start do
            prompt "Continue?"
            fallback "Unknown event."
            on :go do
              reply "Going."
              goto :missing
            end
          end
        end
      end
      """)
    end
  end
end
