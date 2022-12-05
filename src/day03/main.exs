defmodule Day03 do
  def priority(c) do
    {x, _} = Integer.parse(c, 36)

    if String.upcase(c) == c do
      x + 17
    else
      x - 9
    end
  end

  def doubled_badge(list) do
    half_size = div(Enum.count(list), 2)

    [x] =
      Enum.chunk_every(list, half_size)
      |> Enum.map(&MapSet.new/1)
      |> Enum.reduce(&MapSet.intersection/2)
      |> MapSet.to_list()

    x
  end

  def common_badge(sets) do
    sets
    |> Enum.reduce(&MapSet.intersection/2)
    |> MapSet.to_list()
    |> Enum.at(0)
  end
end

elves =
  File.read!("puzzle_input/day03")
  |> String.trim()
  |> String.split(~r/\R/, trim: True)
  |> Enum.map(fn x -> Enum.map(String.graphemes(x), &Day03.priority/1) end)

p1 =
  Enum.map(elves, &Day03.doubled_badge/1)
  |> Enum.sum()

p2 =
  Enum.map(elves, &MapSet.new/1)
  |> Enum.chunk_every(3)
  |> Enum.map(&Day03.common_badge/1)
  |> Enum.sum()

IO.puts("part 1: #{p1}")
IO.puts("part 2: #{p2}")
