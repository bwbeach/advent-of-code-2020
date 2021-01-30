defmodule ElixirAdvent do
  @moduledoc """
  Documentation for `ElixirAdvent`.
  """

  @doc """
  Hello world.

  ## Examples

      iex> ElixirAdvent.hello()
      :world

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
    |> runner.()
    |> print_result(file_path)
  end

  def get_runner(day_number) do
    case day_number do
      1 -> &Day1.run/1
      _ -> &day_not_implemented/1
    end
  end

  def print_result(result, file_path) do
    IO.puts("#{file_path}: #{result}")
  end

  def day_not_implemented(_) do
    "day not implemented"
  end
end
