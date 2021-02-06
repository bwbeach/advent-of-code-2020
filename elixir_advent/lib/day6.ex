defmodule Day6 do
  def run(lines) do
    "part1: #{part1(lines)}   part2: #{part2(lines)}"
  end

  def part1(lines) do
    lines
    |> AdventUtil.split_at_blank_lines()
    |> Enum.map(&collect_unique_graphemes/1)
    |> Enum.map(&MapSet.size/1)
    |> Enum.sum()
  end

  def part2(lines) do
    lines
    |> AdventUtil.split_at_blank_lines()
    |> Enum.map(&all_answered_yes/1)
    |> Enum.map(&MapSet.size/1)
    |> Enum.sum()
  end

  def all_answered_yes(lines) do
    for line <- lines,
        one_set = collect_unique_graphemes([line]),
        reduce: nil do
      acc ->
        case acc do
          nil -> one_set
          _ -> MapSet.intersection(acc, one_set)
        end
    end
  end

  def collect_unique_graphemes(lines) do
    for line <- lines,
        grapheme <- String.graphemes(line),
        into: MapSet.new() do
      grapheme
    end
  end
end
