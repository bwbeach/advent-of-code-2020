defmodule Day5 do
  def run(lines) do
    with [p2] = part2(lines) do
      "part1: #{part1(lines)}   part2: #{p2}"
    end
  end

  def part1(lines) do
    lines
    |> Enum.map(&parse_seat/1)
    |> Enum.max()
  end

  def part2(lines) do
    with seats = seat_set(lines) do
      for s <- seats,
          MapSet.member?(seats, s + 2),
          !MapSet.member?(seats, s + 1) do
        s + 1
      end
    end
  end

  def seat_set(lines) do
    for code <- lines,
        into: MapSet.new() do
      parse_seat(code)
    end
  end

  @doc """
  Parses one seat code and turns it into a seat id.

  ## Examples

    iex> Day5.parse_seat("BFFFBBFRRR")
    567

    iex> Day5.parse_seat("FFFBBBFRRR")
    119

    iex> Day5.parse_seat("BBFFBBFRLL")
    820
  """
  def parse_seat(code) do
    code
    |> String.replace("B", "1")
    |> String.replace("F", "0")
    |> String.replace("R", "1")
    |> String.replace("L", "0")
    |> String.to_integer(2)
  end
end
