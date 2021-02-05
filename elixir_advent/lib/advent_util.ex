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
end
