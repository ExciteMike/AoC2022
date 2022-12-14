defmodule Day14 do
  @moduledoc "AoC2022 Day14 functions"

  @spec drop_sand(map, atom, integer) :: {map, integer}
  def drop_sand(blocked, endcond, endy, x \\ 500, y \\ 0) do
    if Map.has_key?(blocked, {x, y}) do
      nil
    else
      case endcond do
        :floor when y >= endy ->
          {blocked, {x, y}}

        :void when y >= endy ->
          nil

        _ ->
          next =
            Enum.find([x, x - 1, x + 1], nil, fn x ->
              !Map.has_key?(blocked, {x, y + 1})
            end)

          case next do
            nil -> {blocked, {x, y}}
            x -> drop_sand(blocked, endcond, endy, x, y + 1)
          end
      end
    end
  end

  def play(blocked, endcond, endy, count \\ 0) do
    case drop_sand(blocked, endcond, endy) do
      nil ->
        {blocked, count}

      {blocked, key} ->
        play(Map.put(blocked, key, :sand), endcond, endy, count + 1)
    end
  end
end

blocked =
  for line <-
        File.read!("puzzle_input/day14")
        |> String.trim_trailing()
        |> String.split(~r/\R/, trim: True) do
    coords =
      String.split(line, " -> ")
      |> Enum.map(fn s ->
        String.split(s, ",")
        |> Enum.map(fn s -> String.to_integer(s) end)
      end)

    for {[x1, y1], [x2, y2]} <- Stream.zip(coords, Stream.drop(coords, 1)) do
      {x1, x2} =
        if x1 <= x2 do
          {x1, x2}
        else
          {x2, x1}
        end

      {y1, y2} =
        if y1 <= y2 do
          {y1, y2}
        else
          {y2, y1}
        end

      for y <- y1..y2, x <- x1..x2 do
        {{x, y}, :rock}
      end
    end
  end
  |> Stream.concat()
  |> Stream.concat()
  |> Map.new()

{{_, lowest}, _} = Enum.max(blocked, fn {{_, y1}, _}, {{_, y2}, _} -> y1 >= y2 end)

{blocked, p1} = Day14.play(blocked, :void, lowest + 1)
{_, p2} = Day14.play(blocked, :floor, lowest + 12)

# 913, 30762
IO.puts("part 1: #{p1}\npart 2: #{p2}")
