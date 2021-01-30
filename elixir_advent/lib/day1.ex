defmodule Day1 do
  @moduledoc """
  Finds N numbers that sum to 2020 and returns their product.
  """

  def run(lines) do
    with numbers = parse_input(lines),
         [part1] = find_numbers_that_sum(2020, 2, [], numbers),
         [part2] = find_numbers_that_sum(2020, 3, [], numbers) do
      "part1: #{product(part1)}   part2: #{product(part2)}"
    end
  end

  def parse_input(lines) do
    lines
    |> Enum.map(&String.trim/1)
    |> Enum.map(&String.to_integer/1)
  end

  def find_numbers_that_sum(0, 0, so_far, _) do
    [so_far]
  end

  def find_numbers_that_sum(_, 0, _, _) do
    []
  end

  def find_numbers_that_sum(_, _, _, []) do
    []
  end

  # TODO: check that 0 < target
  def find_numbers_that_sum(target, count, so_far, [head | tail]) do
    with with_head = find_numbers_that_sum(target - head, count - 1, [head | so_far], tail),
         sans_head = find_numbers_that_sum(target, count, so_far, tail) do
      with_head ++ sans_head
    end
  end

  def product(numbers) do
    Enum.reduce(numbers, &(&1 * &2))
  end

end
