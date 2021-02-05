defmodule Day4Test do
  use ExUnit.Case
  doctest Day4

  test "parse small input do" do
    with lines = [
           "one:alfa two:bravo",
           "",
           "one:charlie",
           "three:delta"
         ],
         first = %{one: "alfa", two: "bravo"},
         second = %{one: "charlie", three: "delta"},
         expected = [first, second] do
      assert Day4.parse_input(lines) == expected
    end
  end
end
