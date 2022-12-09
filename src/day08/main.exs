defmodule Day08 do
  def sight_line(xs, ys, heights) do
    for x <- xs, y <- ys do
      heights
      |> Map.get({x, y})
    end
  end

  def sight_lines({x, y}, heights, grid_size) do
    [
      sight_line(x..x, (y - 1)..0//-1, heights),
      sight_line(x..x, (y + 1)..(grid_size - 1)//1, heights),
      sight_line((x - 1)..0//-1, y..y, heights),
      sight_line((x + 1)..(grid_size - 1)//1, y..y, heights)
    ]
  end

  def sight_distance(hs, h) do
    case Enum.find_index(hs, &(&1 >= h)) do
      nil -> Enum.count(hs)
      i -> i + 1
    end
  end
end

{input, grid_size} = {File.read!("puzzle_input/day08"), 99}

heights =
  input
  |> String.trim_trailing()
  |> String.split(~r/\R+/, trim: True)
  |> Stream.with_index()
  |> Stream.flat_map(fn {line, y} ->
    String.graphemes(line)
    |> Stream.with_index()
    |> Enum.map(fn {s, x} ->
      {h, _} = Integer.parse(s)
      {{x, y}, h}
    end)
  end)
  |> Map.new()

sight_lines =
  Map.new(heights, fn {xy, h} ->
    {xy, {Day08.sight_lines(xy, heights, grid_size), h}}
  end)

p1 =
  Enum.count(sight_lines, fn {_, {lines, h}} ->
    Enum.any?(lines, fn sight_line ->
      Enum.all?(sight_line, fn x -> x < h end)
    end)
  end)

IO.puts("part 1: #{p1}")

p2 =
  Enum.map(sight_lines, fn {_, {lines, h}} ->
    Enum.map(lines, fn line ->
      Day08.sight_distance(line, h)
    end)
    |> Enum.product()
  end)
  |> Enum.max()

IO.puts("part 2: #{p2}")
