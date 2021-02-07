defmodule AdventUtilTest do
  use ExUnit.Case
  doctest AdventUtil

  test "conway life" do
    with board = MapSet.new([2, 3, 4, 6]),
         neighbor_deltas = [-1, 1],
         add = &(&1 + &2),
         rule = fn was_alive, count ->
           (was_alive and count == 1) or (!was_alive and count == 2)
         end do
      assert AdventUtil.conway_life(board, neighbor_deltas, add, rule) == MapSet.new([2, 4, 5])
    end
  end
end
