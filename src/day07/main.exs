defmodule Day07 do
  def all_uniq(s) do
    String.graphemes(s)
    |> Enum.uniq()
    |> Enum.count() ==
      String.length(s)
  end

  def detect(input, n) do
    {_, i} =
      Day06.windows(input, n)
      |> Stream.with_index()
      |> Stream.filter(fn {s, _} ->
        Day06.all_uniq(s)
      end)
      |> Enum.at(0)

    i + n
  end

  def cmd({cd, sizes}, tokens) do
    case tokens do
      ["cd", ".."] -> {tl(cd), sizes}
      ["cd", "/"] -> {[], sizes}
      ["cd", name] -> {[name|cd], sizes}
      ["ls"] -> {cd, sizes}
      [s, _] when String.match?(s, ~r/\d*/) ->
    end
  end
end

input =
  File.read!("puzzle_input/day07")
  |> String.split(~r/\R/, trim: True)
  |> Enum.reduce({[], %{}}, fn line, state ->
    case String.split(line, ~r/\s/) do
      ["$"|rest] -> Day07.cmd(state, rest)
      ["dir", name] ->
        {cd, sizes} = state
        {cd, Map.put(sizes, name, 0)}
    end
  end)
  |> IO.inspect()

IO.puts("part 1: #{0}")
IO.puts("part 2: #{0}")
