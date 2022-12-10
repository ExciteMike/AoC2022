{xs, _} =
  File.read!("puzzle_input/day10")
  |> String.trim_trailing()
  |> String.split(~r/\R+/, trim: True)
  |> Enum.flat_map_reduce(1, fn line, x ->
    if String.starts_with?(line, "a") do
      {v, _} = Integer.parse(String.slice(line, 5..99))
      {[x, x + v], x + v}
    else
      {[x], x}
    end
  end)

xs = Enum.with_index([1 | [1 | xs]])

p1 =
  for {x, i} <- Enum.slice(xs, 20..220//40), reduce: 0 do
    acc -> acc + x * i
  end

p2 =
  Enum.reduce(xs, "", fn {x, i}, acc ->
    col = rem(i, 40)

    s =
      if (col - x) in 0..2 do
        "[]"
      else
        "  "
      end

    cond do
      i >= 6 * 40 -> acc
      col == 39 -> acc <> s <> "\n"
      True -> acc <> s
    end
  end)

IO.puts("part 1: #{p1}")
IO.write("part 2: \n#{p2}")
