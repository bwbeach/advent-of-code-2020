defmodule ElixirAdvent.CLI do
  @moduledoc """
  Parse the command line and dispatch to the module
  handling the requested day.
  """

  def run(argv) do
    IO.puts(Day1.run("samples/day1_sample.txt", File.stream!("samples/day1_sample.txt")))
  end

  def run_argv() do
    run(System.argv())
  end
end
