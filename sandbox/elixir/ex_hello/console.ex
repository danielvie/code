defmodule Hello.Console do
  @moduledoc "Console I/O kept separate from the questionnaire's transition rules."

  def run(module) do
    IO.puts("Hello, world! Type restart or quit at any question.")
    loop(module, module.new())
  end

  defp loop(module, machine) do
    IO.puts("\n[state: #{machine.state}]")

    unless module.terminal?(machine) do
      IO.puts(module.prompt(machine))

      case IO.gets("> ") do
        :eof ->
          IO.puts("\nInput closed. Goodbye!")

        {:error, reason} ->
          IO.puts(:stderr, "Could not read input: #{inspect(reason)}")

        input ->
          {next_machine, message} = module.transition(machine, input)
          IO.puts(message)
          loop(module, next_machine)
      end
    end
  end
end
