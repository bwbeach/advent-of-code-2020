defmodule Day4 do
  def run(lines) do
    with input = parse_input(lines),
         part1 = Enum.count(input, &part1_valid/1),
         part2 = Enum.count(input, &part2_valid/1) do
      "part1: #{part1}   part2: #{part2}"
    end
  end

  def part1_valid(passport) do
    case passport do
      %{byr: _, iyr: _, eyr: _, hgt: _, hcl: _, ecl: _, pid: _} ->
        true

      _ ->
        false
    end
  end

  def part2_valid(passport) do
    case passport do
      %{byr: byr, iyr: iyr, eyr: eyr, hgt: hgt, hcl: hcl, ecl: ecl, pid: pid} ->
        number_in_range(byr, ~r/^([0-9]{4})$/, 1920, 2002) and
          number_in_range(iyr, ~r/^([0-9]{4})$/, 2010, 2020) and
          number_in_range(eyr, ~r/^([0-9]{4})$/, 2020, 2030) and
          height_valid(hgt) and
          String.match?(hcl, ~r/^#[0-9a-f]{6}$/) and
          Enum.member?(~w{amb blu brn gry grn hzl oth}, ecl) and
          String.match?(pid, ~r/^[0-9]{9}$/)

      _ ->
        false
    end
  end

  def height_valid(hgt) do
    number_in_range(hgt, ~r/^([0-9]+)cm$/, 150, 193) or
      number_in_range(hgt, ~r/^([0-9]+)in$/, 59, 76)
  end

  @doc """
  Checks that the string matches the pattern, and the first capture
  group in a number in the given range (inclusive).

  ## Examples:

    iex> Day4.number_in_range("a47b", ~r/^a([47]+)b$/, 0, 50)
    true
    iex> Day4.number_in_range("a77b", ~r/^a([47]+)b$/, 0, 50)
    false
    iex> Day4.number_in_range("49", ~r/([0-9]+)/, 50, 50)
    false
    iex> Day4.number_in_range("50", ~r/([0-9]+)/, 50, 50)
    true
    iex> Day4.number_in_range("51", ~r/([0-9]+)/, 50, 50)
    false

  """
  def number_in_range(text, pattern, low, high) do
    case Regex.run(pattern, text) do
      nil ->
        false

      [_, num_str] ->
        with num = String.to_integer(num_str) do
          low <= num and num <= high
        end
    end
  end

  @doc """
  Takes the textual input an converts it into a list
  of maps.  Each map represents one passport, mapping
  field name to value.
  """
  def parse_input(lines) do
    lines
    |> AdventUtil.split_at_blank_lines()
    |> Enum.map(&parse_one_passport/1)
  end

  def parse_one_passport(lines) do
    for line <- lines,
        item <- String.split(line),
        [field, value] = String.split(item, ":"),
        into: %{} do
      {String.to_atom(field), value}
    end
  end
end
