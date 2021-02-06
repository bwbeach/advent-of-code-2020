defmodule ElixirAdvent do
  @moduledoc """
  Documentation for `ElixirAdvent`.
  """

  def run_day(day_number) do
    case Path.wildcard("samples/day#{day_number}_*") do
      [] ->
        IO.puts("no input files")

      paths ->
        with runner = get_runner(day_number) do
          Enum.each(paths, &run_file(&1, runner))
        end
    end
  end

  def run_file(file_path, runner) do
    file_path
    |> File.stream!()
    |> Enum.map(&String.trim/1)
    |> runner.()
    |> print_result(file_path)
  end

  @spec get_runner(integer) :: ([String] -> String)
  def get_runner(day_number) do
    case day_number do
      1 -> &Day1.run/1
      2 -> &Day2.run/1
      3 -> &Day3.run/1
      4 -> &Day4.run/1
      5 -> &Day5.run/1
      6 -> &Day6.run/1
      7 -> &Day7.run/1
      _ -> &day_not_implemented/1
    end
  end

  def print_result(result, file_path) do
    IO.puts("#{file_path}: #{result}")
  end

  @doc """
  Hello world.

  ## Examples

      iex> ElixirAdvent.day_not_implemented("")
      "day not implemented"

  """
  def day_not_implemented(_) do
    "day not implemented"
  end
end
