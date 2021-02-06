defmodule Day7 do
  def run(lines) do
    with rules = parse_rules(lines),
         p1 = part1(rules),
         p2 = bags_inside("shiny gold", rules) do
      "part1: #{p1}   part2: #{p2}"
    end
  end

  def part1(rules) do
    rules
    |> Enum.count(fn {color, _} -> can_contain(color, "shiny gold", rules) end)
  end

  @doc """
  Returns the number of bags inside a bag of a given
  color, not counting the outer bag itself.
  """
  def bags_inside(color, rules) do
    # For one item in a rule, how many bags?
    # This includes the bag inself and everything
    # inside it.
    bags_for_rule_item = fn {count, inner_color} ->
      count * (1 + bags_inside(inner_color, rules))
    end

    # Sum the bags for each item in the rule.
    rules[color]
    |> Enum.map(bags_for_rule_item)
    |> Enum.sum()
  end

  def can_contain(outer_color, inner_color, rules) do
    # Can one rule item contain the color, either
    # by specifying that color directly, or by
    # specifying a color that contains it?
    rule_item_can_contain_it = fn {_, content_color} ->
      content_color == inner_color or
      can_contain(content_color, inner_color, rules)
    end

    rules[outer_color]
    |> Enum.any?(rule_item_can_contain_it)
  end

  @spec parse_rules([String.t()]) :: %{String.t() => [{integer, String.t()}]}
  def parse_rules(lines) do
    for line <- lines,
        into: %{} do
      parse_rule(line)
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
  @spec parse_rule(String.t()) :: {String.t(), [{integer, String.t()}]}
  def parse_rule(line) do
    with [_, lhs, rhs] = Regex.run(~r{^(.*) bags contain (.*)}, line) do
      {lhs, parse_rhs(rhs)}
    end
  end

  @spec parse_rhs(String.t()) :: [{integer, String.t()}]
  def parse_rhs(rhs) do
    if String.contains?(rhs, "no other bag") do
      []
    else
      for item <- String.split(rhs, ", ") do
        with [_, count_str, color] = Regex.run(~r{([0-9]+) (.*) bag}, item) do
          {String.to_integer(count_str), color}
        end
      end
    end
  end
end
