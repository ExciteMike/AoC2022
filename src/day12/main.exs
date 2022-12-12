defmodule Day12 do
  def neighbors(heights, pos = {x, y}) do
    h = Map.get(heights, pos)

    Enum.filter([{x + 1, y}, {x, y + 1}, {x, y - 1}, {x - 1, y}], fn pos ->
      case Map.get(heights, pos) do
        nil -> false
        h2 -> h < h2 + 2
      end
    end)
  end

  def dijkstra_map(ds, [], _) do
    ds
  end

  def dijkstra_map(distances, [{pos, d} | q], heights) do
    ns =
      neighbors(heights, pos)
      |> Enum.reject(fn neighbor ->
        Map.has_key?(distances, neighbor)
      end)
      |> Enum.map(fn neighbor ->
        {neighbor, d + 1}
      end)

    dijkstra_map(Map.merge(distances, Map.new(ns)), q ++ ns, heights)
  end
end

chargrid =
  File.read!("puzzle_input/day12")
  |> String.trim_trailing()
  |> String.split(~r/\R/, trim: True)
  |> Stream.with_index()
  |> Stream.flat_map(fn {line, y} ->
    String.codepoints(line)
    |> Stream.with_index()
    |> Stream.map(fn {c, x} ->
      {{x, y}, c}
    end)
  end)
  |> Map.new()

{start_pos, _} = Enum.find(chargrid, fn {_, c} -> c == "S" end)
{end_pos, _} = Enum.find(chargrid, fn {_, c} -> c == "E" end)

heights =
  Map.new(chargrid, fn {pos, c} ->
    h =
      case c do
        "S" -> 10
        "E" -> 35
        c -> String.to_integer(c, 36)
      end

    {pos, h}
  end)

distances = Day12.dijkstra_map(%{end_pos => 0}, [{end_pos, 0}], heights)

# 447
IO.puts("part 1: #{Map.get(distances, start_pos)}")

p2 =
  Enum.filter(heights, fn {_, h} -> h == 10 end)
  |> Enum.map(fn {pos, _} -> Map.get(distances, pos) end)
  |> Enum.min()

# 446
IO.puts("part 2: #{p2}")
