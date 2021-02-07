defmodule AdventUtil do
  @doc """
  Given a list of lines, break at blank lines into multiple lists of lines.

  ## Examples:

    iex> AdventUtil.split_at_blank_lines(["a", "b", "", "", "c", "d"])
    [["a", "b"], ["c", "d"]]
  """
  def split_at_blank_lines(lines) do
    lines
    |> Enum.chunk_by(&(&1 == ""))
    |> Enum.filter(fn [first | _] -> first != "" end)
  end

  @doc """
  Runs one iteration of generalized Conway's life.  Input is:

    # A `board` which is a MapSet of positions of live cells
    # A set of `neighbor_deltas`, each of which can be added
      to a cell position to get a neighboring cell position.
    # An `add` function to add a cell position and a delta
    # A `rule` function that takes a number of live neighbors, and
      a boolean saying whether the given cell is currently live
      and returns a boolean saying whether the new state is live.

  NOTE: Does not support rules that say cells are alive when they
  have no live neighbers, even if they were alive before.
  """
  def conway_life(board, neighbor_deltas, add, rule) do
    for {cell, count} <- neighbor_counts(board, neighbor_deltas, add),
        (was_alive = MapSet.member?(board, cell)) or true,
        rule.(was_alive, count),
        into: MapSet.new() do
      cell
    end
  end

  @doc """
  Computes a map from cell name to number of live neighbors.
  Includes every cell that has neighbors *and* every cell that
  is alive now.

  NOTE: Cells that were alive, but have neighbor counts of
  zero are not included in the result.

  ## Examples:

    iex> AdventUtil.neighbor_counts(MapSet.new([2, 3, 4, 6]), [-1, 1], &(&1 + &2))
    %{1 => 1, 2 => 1, 3 => 2, 4 => 1, 5 => 2, 7 => 1}
  """
  def neighbor_counts(board, neighbor_deltas, add) do
    for live_cell <- board,
        delta <- neighbor_deltas,
        neighbor = add.(live_cell, delta),
        reduce: %{} do
      acc ->
        Map.put(acc, neighbor, Map.get(acc, neighbor, 0) + 1)
    end
  end
end
