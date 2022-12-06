defmodule Day06 do
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

  def windows(s, n) do
    n..String.length(s)
    |> Stream.map(fn i ->
      String.slice(s, (i - n)..(i - 1))
    end)
  end
end

input =
  File.read!("puzzle_input/day06")
  |> String.trim_trailing()

IO.puts("part 1: #{Day06.detect(input, 4)}")
IO.puts("part 2: #{Day06.detect(input, 14)}")
