defmodule ElixirAdvent.CLI do
  @moduledoc """
  Parse the command line and dispatch to the module
  handling the requested day.
  """

  def run(argv) do
    with [day_str] = argv,
         day_number = String.to_integer(day_str) do
      IO.puts(ElixirAdvent.run_day(day_number))
    end
  end

  def run_argv() do
    run(System.argv())
  end
end
