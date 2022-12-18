defmodule Day18 do
  @moduledoc "AoC2022 Day18 functions"

  def neighbors(x, y, z) do
    [
      {x - 1, y, z},
      {x + 1, y, z},
      {x, y - 1, z},
      {x, y + 1, z},
      {x, y, z - 1},
      {x, y, z + 1}
    ]
  end

  defp bounds(coords) do
    for {x, y, z} <- coords, reduce: {{0, 0}, {0, 0}, {0, 0}} do
      {{x0, x1}, {y0, y1}, {z0, z1}} ->
        {
          {min(x0, x - 1), max(x1, x + 1)},
          {min(y0, y - 1), max(y1, y + 1)},
          {min(z0, z - 1), max(z1, z + 1)}
        }
    end
  end

  def exterior(cubes) do
    exterior(cubes, [{0, 0, 0}], MapSet.new(), bounds(cubes))
  end

  defp exterior(_, [], result, _) do
    result
  end

  defp exterior(cubes, [{x, y, z} | stack], set, bounds = {{x0, x1}, {y0, y1}, {z0, z1}}) do
    new =
      neighbors(x, y, z)
      |> Enum.filter(fn p = {x, y, z} ->
        x in x0..x1 and y in y0..y1 and z in z0..z1 and p not in cubes and p not in set
      end)

    stack =
      Enum.reduce(new, stack, fn p, stack ->
        [p | stack]
      end)

    set =
      Enum.reduce(new, set, fn p, set ->
        MapSet.put(set, p)
      end)

    exterior(cubes, stack, set, bounds)
  end
end

cubes =
  for line <-
        File.read!("puzzle_input/day18")
        |> String.trim_trailing()
        |> String.split(~r/\R/, trim: True) do
    [x, y, z] =
      String.split(line, ",")
      |> Enum.map(fn s -> String.to_integer(s) end)

    {x, y, z}
  end
  |> MapSet.new()

neighbors = Enum.flat_map(cubes, fn {x, y, z} -> Day18.neighbors(x, y, z) end)
p1 = Enum.count(neighbors, fn p -> p not in cubes end)
exterior = Day18.exterior(cubes)
p2 = p1 - Enum.count(neighbors, fn p -> p not in cubes and p not in exterior end)

# 4288, 2494
IO.puts("part 1: #{p1}\npart 2: #{p2}")
