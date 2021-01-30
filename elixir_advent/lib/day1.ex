defmodule Day1 do
  @moduledoc """
  Finds two numbers that sum to 2020 and prints their product.
  """

  def run(_file_name, lines) do
    with numbers =
           lines
           |> Enum.map(&String.trim/1)
           |> Enum.map(&String.to_integer/1),
         {:ok, product} = find_answer(numbers) do
      product
    end
  end

  def find_answer([]) do
    {:no_answer, 0}
  end

  def find_answer([head | tail]) do
    case find_answer_with(head, tail) do
      {:ok, product} ->
        { :ok, product }

      {:no_answer, _} ->
        find_answer(tail)
    end
  end

  def find_answer_with(first, [head | tail]) do
    if first + head == 2020 do
      { :ok, first * head }
    else
      find_answer_with(first, tail)
    end
  end

  def find_answer_with(_, []) do
    {:no_answer, 0}
  end
end
