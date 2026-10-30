Code.require_file("state_machine.ex", __DIR__)
Code.require_file("console.ex", __DIR__)

defmodule Hello.Questionnaire do
  @moduledoc "A questionnaire described using a local state-machine DSL."
  import Hello.StateMachine, only: [defmachine: 1]

  defmachine do
    initial(:ask_name)

    track(:name, default: nil)
    track(:experience, default: nil)

    global do
      on :restart do
        update(fn _input, _tracks -> %{name: nil, experience: nil} end)
        reply("Starting over.")
        goto(:ask_name)
      end

      on :quit do
        reply("Goodbye!")
        goto(:quit)
      end
    end

    state :ask_name do
      prompt("What is your name?")
      fallback("Please enter a non-empty name.")

      on :answer do
        guard(fn input, _tracks -> input != "" end)
        update(fn input, tracks -> %{tracks | name: input} end)
        reply("Name saved.")
        goto(:ask_experience)
      end
    end

    state :ask_experience do
      prompt(fn tracks -> "Hi, #{tracks.name}! Have you used Elixir before? (yes/no)" end)
      fallback("Please answer yes or no.")

      on :answer do
        guard(fn input, _tracks -> String.downcase(input) in ["yes", "no"] end)
        update(fn input, tracks -> %{tracks | experience: String.downcase(input)} end)
        reply("Experience saved.")
        goto(:confirm)
      end
    end

    state :confirm do
      prompt(fn tracks ->
        "Name: #{tracks.name}, Elixir experience: #{tracks.experience}. " <>
          "Confirm? (yes/name/experience)"
      end)

      fallback("Choose yes, name, or experience.")

      on :answer do
        guard(fn input, _tracks -> String.downcase(input) == "yes" end)

        reply(fn tracks ->
          if tracks.experience == "yes" do
            "Hello, #{tracks.name}! Welcome back to Elixir."
          else
            "Hello, #{tracks.name}! Welcome to your first Elixir program."
          end
        end)

        goto(:done)
      end

      on :answer do
        guard(fn input, _tracks -> String.downcase(input) == "name" end)
        reply("Let's update your name.")
        goto(:ask_name)
      end

      on :answer do
        guard(fn input, _tracks -> String.downcase(input) == "experience" end)
        reply("Let's update your experience.")
        goto(:ask_experience)
      end
    end

    terminal(:done)
    terminal(:quit)
  end

  def prompt(machine), do: Hello.StateMachine.prompt(__MODULE__, machine)
  def terminal?(machine), do: Hello.StateMachine.terminal?(__MODULE__, machine)

  # Translate console text into events. The engine itself does no console I/O.
  def transition(machine, input) do
    input = String.trim(input)

    event =
      case String.downcase(input) do
        "restart" -> :restart
        "quit" -> :quit
        _ -> :answer
      end

    Hello.StateMachine.transition(__MODULE__, machine, event, input)
  end

  def run, do: Hello.Console.run(__MODULE__)
end
