defmodule Day7 do
  def run(lines) do
    with rules = parse_rules(lines) do
      part1(rules)
    end
  end

  def part1(rules) do
    rules
    |> Enum.count(fn {color, _} -> can_contain(color, "shiny gold", rules) end)
  end

  def parse_rules(lines) do
    for line <- lines,
        into: %{} do
      parse_rule(line)
    end
  end

  def can_contain(outer_color, inner_color, rules) do
    with contents = rules[outer_color] do
      Enum.any?(
        contents,
        fn {_, content_color} ->
          content_color == inner_color or
            can_contain(content_color, inner_color, rules)
        end
      )
    end
  end

  @doc """
  Parses one contents line

  ## Examples

    iex> Day7.parse_rule("light red bags contain 1 bright white bag, 2 muted yellow bags.")
    {"light red", [{1, "bright white"}, {2, "muted yellow"}]}

    iex> Day7.parse_rule("faded blue bags contain no other bags.")
    {"faded blue", []}
  """
  def parse_rule(line) do
    with [_, lhs, rhs] = Regex.run(~r{^(.*) bags contain (.*)}, line) do
      {lhs, parse_rhs(rhs)}
    end
  end

  def parse_rhs(rhs) do
    case Regex.run(~r{no other bag}, rhs) do
      nil ->
        for item <- String.split(rhs, ", ") do
          with [_, count_str, color] = Regex.run(~r{([0-9]+) (.*) bag}, item) do
            {String.to_integer(count_str), color}
          end
        end

      _ ->
        []
    end
  end
end
