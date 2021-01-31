defmodule Day2Test do
  use ExUnit.Case
  doctest Day2

  test "parse input line" do
    assert Day2.parse_line("1-3 b: cdefg") == { 1, 3, "b", "cdefg" }
  end

  test "parse input line with multi-digit numbers" do
    assert Day2.parse_line("11-33 b: cdefg") == { 11, 33, "b", "cdefg" }
  end

  test "count chars in string" do
    assert Day2.count_char("a", "aloha") == 2
  end

  test "part 2 allows example 1" do
    assert Day2.part2_allows({1, 3, "a", "abcde"}) == true
  end

  test "part 2 allows example 2" do
    assert Day2.part2_allows({5, 13, "s", "brhsssnfcndsh"}) == true
  end
end
