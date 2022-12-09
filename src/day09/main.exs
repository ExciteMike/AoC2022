defmodule Day09 do
  def sign(x) do
    cond do
      x > 0 -> 1
      x == 0 -> 0
      x < 0 -> -1
    end
  end

  def follow(path) do
    Stream.transform(path, {0, 0}, fn {hx, hy}, {tx, ty} ->
      if hx - 1 <= tx && tx <= hx + 1 && hy - 1 <= ty && ty <= hy + 1 do
        {[], {tx, ty}}
      else
        p = {tx + sign(hx - tx), ty + sign(hy - ty)}
        {[p], p}
      end
    end)
    |> Enum.to_list()
  end
end

path0 =
  File.read!("puzzle_input/day09")
  |> String.trim_trailing()
  |> String.split(~r/\R+/, trim: True)
  |> Enum.flat_map(fn s ->
    [dir, dist] = String.split(s)
    {dist, _} = Integer.parse(dist)

    step =
      case dir do
        "U" -> {0, 1}
        "D" -> {0, -1}
        "L" -> {-1, 0}
        "R" -> {1, 0}
      end

    Stream.cycle([step])
    |> Stream.take(dist)
  end)
  |> Stream.transform({0, 0}, fn {dx, dy}, {x, y} ->
    p = {x + dx, y + dy}
    {[p], p}
  end)

path1 = Day09.follow(path0)

p1 = Enum.count(Stream.uniq([{0, 0} | path1]))
IO.puts("part 1: #{p1}")

path9 =
  Enum.reduce(2..9, path1, fn _, path ->
    Day09.follow(path)
  end)

p2 = Enum.count(Stream.uniq([{0, 0} | path9]))
IO.puts("part 2: #{p2}")
