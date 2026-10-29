defmodule Hello.StateMachine do
  @moduledoc """
  A small teaching DSL, not an implementation of the Accord library.

  Rules match an event and then evaluate their guard. The first passing rule
  wins, with global rules checked before state-local rules. Updates return a
  complete tracks map. Replies receive the updated tracks. Terminal states
  ignore all events, including global events.
  """

  defmacro defmachine(do: block) do
    declarations = entries(block, [:initial, :track, :global, :state, :terminal])
    initial = declarations |> Keyword.fetch!(:initial) |> hd()

    tracks =
      for [name, [default: value]] <- Keyword.get_values(declarations, :track), do: {name, value}

    global = Keyword.get(declarations, :global, [[do: nil]])
    [[do: global_block]] = global
    global_rules = rules(entries(global_block, [:on]))

    states =
      for [name, [do: body]] <- Keyword.get_values(declarations, :state) do
        fields = entries(body, [:prompt, :fallback, :on])
        prompt = fields |> Keyword.fetch!(:prompt) |> hd()
        fallback = fields |> Keyword.fetch!(:fallback) |> hd()

        {name,
         quote do
           %{
             terminal: false,
             prompt: unquote(prompt),
             fallback: unquote(fallback),
             rules: unquote(rules(fields))
           }
         end}
      end

    terminals =
      for [name] <- Keyword.get_values(declarations, :terminal) do
        {name, quote(do: %{terminal: true})}
      end

    all_states = states ++ terminals
    names = Enum.map(all_states, &elem(&1, 0))

    if length(names) != length(Enum.uniq(names)) or initial not in names do
      raise ArgumentError, "states must be unique and initial must name a declared state"
    end

    # Inspect the declarations, not generated code, to catch misspelled destinations.
    bodies = [
      global_block
      | Enum.map(Keyword.get_values(declarations, :state), fn [_, [do: body]] -> body end)
    ]

    for body <- bodies,
        {:on, [_, [do: rule]]} <- entries(body, [:prompt, :fallback, :on]),
        {:goto, [destination]} <- entries(rule, [:guard, :update, :reply, :goto]) do
      if destination not in names,
        do: raise(ArgumentError, "unknown destination: #{inspect(destination)}")
    end

    definitions =
      for {name, spec} <- all_states do
        quote do
          def state_spec(unquote(name)), do: unquote(spec)
        end
      end

    quote do
      def new, do: Map.put(unquote({:%{}, [], tracks}), :state, unquote(initial))
      def global_rules, do: unquote(global_rules)
      unquote_splicing(definitions)
    end
  end

  defp entries(nil, _allowed), do: []

  defp entries(block, allowed) do
    expressions =
      case block do
        {:__block__, _, expressions} -> expressions
        expression -> [expression]
      end

    Enum.map(expressions, fn
      {name, _, args} = expression when is_list(args) ->
        if name in allowed do
          {name, args}
        else
          raise ArgumentError, "unexpected DSL expression: #{Macro.to_string(expression)}"
        end

      expression ->
        raise ArgumentError, "unexpected DSL expression: #{Macro.to_string(expression)}"
    end)
  end

  defp rules(fields) do
    for [event, [do: body]] <- Keyword.get_values(fields, :on) do
      rule = entries(body, [:guard, :update, :reply, :goto])
      guard = rule |> Keyword.get(:guard, [quote(do: fn _, _ -> true end)]) |> hd()
      update = rule |> Keyword.get(:update, [quote(do: fn _, tracks -> tracks end)]) |> hd()
      reply = rule |> Keyword.fetch!(:reply) |> hd()
      destination = rule |> Keyword.fetch!(:goto) |> hd()

      quote do
        %{
          event: unquote(event),
          guard: unquote(guard),
          update: unquote(update),
          reply: unquote(reply),
          destination: unquote(destination)
        }
      end
    end
  end

  def terminal?(module, machine), do: module.state_spec(machine.state).terminal

  def prompt(module, machine) do
    spec = module.state_spec(machine.state)
    render(spec.prompt, Map.delete(machine, :state))
  end

  def transition(module, machine, event, input) do
    spec = module.state_spec(machine.state)
    tracks = Map.delete(machine, :state)

    if spec.terminal do
      {machine, "This questionnaire has ended."}
    else
      rule =
        Enum.find(module.global_rules() ++ spec.rules, fn rule ->
          rule.event == event and rule.guard.(input, tracks)
        end)

      case rule do
        nil ->
          {machine, render(spec.fallback, tracks)}

        rule ->
          updated = rule.update.(input, tracks)

          if Enum.sort(Map.keys(updated)) != Enum.sort(Map.keys(tracks)) do
            raise ArgumentError, "update must return exactly the declared tracks"
          end

          {Map.put(updated, :state, rule.destination), render(rule.reply, updated)}
      end
    end
  end

  defp render(text, _tracks) when is_binary(text), do: text
  defp render(callback, tracks) when is_function(callback, 1), do: callback.(tracks)
end
