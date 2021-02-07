defmodule Day9 do
  def run(lines) do
    with input = parse_input(lines),
         [p1] = part1(input),
         p2 = part2(p1, input) do
      "part1: #{p1}   part2: #{p2}"
    end
  end

  def part1({preamble_size, input}) do
    for index <- preamble_size..(Kernel.map_size(input) - 1),
        low_index = index - preamble_size,
        high_index = low_index + preamble_size - 1,
        not is_sum_of_two(input[index], low_index, high_index, input) do
      input[index]
    end
  end

  def is_sum_of_two(value, low_index, high_index, data) do
    for index_a <- low_index..(high_index - 1),
        index_b <- (index_a + 1)..high_index,
        value == data[index_a] + data[index_b],
        reduce: false do
      _ -> true
    end
  end

  def part2(target, {_, data}) do
    with subseq = find_subsequence_with_sum(target, data) do
      Enum.min(subseq) + Enum.max(subseq)
    end
  end

  @doc """
  Finds a subsequence of indices into a map, where the values
  sum to a given target.

  ## Examples:

    iex> Day9.find_subsequence_with_sum(6, %{0 => 1, 1 => 2, 2 => 4, 3 => 8})
    [2, 4]
  """
  def find_subsequence_with_sum(target, data) do
    find_subsequence_with_sum(target, 0, 0, data[0], Kernel.map_size(data) - 1, data)
  end

  def find_subsequence_with_sum(target, low, high, sum, max, data) do
    cond do
      target == sum ->
        for i <- low..high do
          data[i]
        end

      sum < target ->
        if high < max do
          find_subsequence_with_sum(target, low, high + 1, sum + data[high + 1], max, data)
        else
          nil
        end

      sum > target ->
        find_subsequence_with_sum(target, low + 1, high, sum - data[low], max, data)
    end
  end

  @spec parse_input(nonempty_maybe_improper_list) :: {integer, any}
  @doc """
  Parse the input, producing two things: the size of the preamble,
  and a map from index to input number.

  ## Examples

    iex> Day9.parse_input(["1", "3", "400"])
    {1, %{0 => 3, 1 => 400}}
  """
  def parse_input([first | rest]) do
    {String.to_integer(first), lines_to_map(rest)}
  end

  def lines_to_map(lines) do
    with swap = fn {a, b} -> {b, a} end do
      lines
      |> Enum.map(&String.to_integer/1)
      |> Enum.with_index()
      |> Enum.map(swap)
      |> Enum.into(%{})
    end
  end
end
