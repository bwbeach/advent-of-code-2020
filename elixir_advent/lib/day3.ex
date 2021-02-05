defmodule Day3 do
  def run(lines) do
    with part1 = product_of_collision_counts(lines, [{3, 1}]),
         part2 = product_of_collision_counts(lines, [{1, 1}, {3, 1}, {5, 1}, {7, 1}, {1, 2}]) do
      "part1: #{part1}   part2: #{part2}"
    end
  end

  def product_of_collision_counts(lines, slopes) do
    for slope <- slopes,
        collisions = count_collisions(lines, slope),
        reduce: 1 do
      product -> product * collisions
    end
  end

  def count_collisions(lines, slope) do
    with {width, height, trees} = parse_input(lines),
         collisions = find_collisions(width, height, trees, slope) do
      Enum.count(collisions)
    end
  end

  def find_collisions(width, height, trees, slope) do
    for loc <- toboggan_positions(width, height, slope),
        MapSet.member?(trees, loc) do
      loc
    end
  end

  @doc """
  Returns all of the positions the toboggan with occupy
  on its way down the hill.

  ## Examples

    iex> Day3.toboggan_positions(5, 5, {3, 1})
    [{0, 0}, {3, 1}, {1, 2}, {4, 3}, {2, 4}]

    iex> Day3.toboggan_positions(5, 5, {1, 2})
    [{0, 0}, {1, 2}, {2, 4}]
  """
  def toboggan_positions(width, height, {dx, dy}) do
    for i <- 0..div(height - 1, dy),
        y = i * dy,
        x = rem(i * dx, width) do
      {x, y}
    end
  end

  @doc """
  Takes an enum of input lines and returns the size of
  the input grid, along with a set of {x,y} positions
  where there are trees.

    { width, height, tree_positions }
  """
  def parse_input(lines) do
    with first_line = Enum.at(lines, 0),
         width = String.length(first_line),
         height = Enum.count(lines),
         trees = parse_trees(lines) do
      {width, height, trees}
    end
  end

  def parse_trees(lines) do
    for {line, y} <- Enum.with_index(lines),
        {"#", x} <- line |> String.graphemes() |> Enum.with_index(),
        into: MapSet.new() do
      {x, y}
    end
  end
end
