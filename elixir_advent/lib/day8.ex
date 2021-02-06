defmodule Day8 do
  def run(lines) do
    with program = parse_program(lines),
         {:loop, part1} = run_program(program, 0, 0, MapSet.new()),
         [fixed_program] = fix_program(program),
         {:finish, part2} = run_program(fixed_program, 0, 0, MapSet.new()) do
      "part1: #{part1}   part2: #{part2}"
    end
  end

  def fix_program(program) do
    for addr_to_change <- Map.keys(program),
        new_program = change_program(program, addr_to_change),
        new_program != nil,
        {:finish, _} <- [run_program(new_program, 0, 0, MapSet.new())] do
      new_program
    end
  end

  @doc """
  Swaps jmp/nop at the given location, if possible, returning a new program.
  Returns nil if not possible.
  """
  def change_program(program, addr_to_change) do
    case program[addr_to_change] do
      {:jmp, n} ->
        Map.put(program, addr_to_change, {:nop, n})

      {:nop, n} ->
        Map.put(program, addr_to_change, {:jmp, n})

      _ ->
        nil
    end
  end

  @doc """
  Runs a program, returning the accumulator just before
  an instruction would be run for the second time.
  """
  def run_program(program, pc, acc, already_run) do
    cond do
      MapSet.member?(already_run, pc) ->
        {:loop, acc}

      pc == Kernel.map_size(program) ->
        {:finish, acc}

      true ->
        with new_already_run = MapSet.put(already_run, pc),
             {new_pc, new_acc} = run_instruction(pc, acc, program[pc]) do
          run_program(program, new_pc, new_acc, new_already_run)
        end
    end
  end

  def run_instruction(pc, acc, instruction) do
    case instruction do
      {:jmp, n} -> {pc + n, acc}
      {:acc, n} -> {pc + 1, acc + n}
      {:nop, _} -> {pc + 1, acc}
    end
  end

  @doc """
  Parses a program, and returns a map from instruction number to instruction.
  """
  def parse_program(lines) do
    swap = fn {a, b} -> {b, a} end

    lines
    |> Enum.map(&parse_line/1)
    |> Enum.with_index()
    |> Enum.map(swap)
    |> Enum.into(%{})
  end

  @spec parse_line(String.t()) :: {atom, integer}
  @doc """
  Parse the instruction on one line, returing a tuple
  of instruction name (an atom) and a number.

  ## Examples

    iex> Day8.parse_line("nop +0")
    {:nop, 0}

    iex> Day8.parse_line("jmp +4")
    {:jmp, 4}

    iex> Day8.parse_line("acc -99")
    {:acc, -99}
  """
  def parse_line(line) do
    with [instruction_str, number_str] = String.split(line) do
      {String.to_atom(instruction_str), String.to_integer(number_str)}
    end
  end
end
