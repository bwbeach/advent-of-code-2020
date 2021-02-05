defmodule Day3Test do
  use ExUnit.Case
  doctest Day3

  test "parse small input" do
    with trees = MapSet.new() |> MapSet.put({2, 0}) |> MapSet.put({1, 1}) do
      assert Day3.parse_input(["..#", ".#."]) == {3, 2, trees}
    end
  end
end
