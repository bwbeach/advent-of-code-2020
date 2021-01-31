defmodule Day2 do

  @input_pattern ~r{([0-9]+)-([0-9]+) ([a-z]): (.*)}

  def run(lines) do
    with part1 = run_part(lines, &part1_allows/1),
         part2 = run_part(lines, &part2_allows/1) do
      "part1: #{part1}   part2: #{part2}"
    end
  end

  def run_part(lines, filter) do
    lines
    |> Enum.map(&parse_line/1)
    |> Enum.count(filter)
  end

  def part1_allows({n1, n2, c, word}) do
    with count = count_char(c, word) do
      n1 <= count && count <= n2
    end
  end

  def part2_allows({n1, n2, c, word}) do
    with first_matches = String.at(word, n1 - 1) == c,
         second_matches = String.at(word, n2 - 1) == c do
      first_matches != second_matches
    end
  end

  def count_char(c, s) do
    String.graphemes(s)
    |> Enum.count(&(&1 == c))
  end

  def parse_line(line) do
      with [_, n1_str, n2_str, c, word] = Regex.run(@input_pattern, line),
           n1 = String.to_integer(n1_str),
           n2 = String.to_integer(n2_str) do
        { n1, n2, c, word }
      end
  end

end
