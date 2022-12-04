blocks =
  File.read!("puzzle_input/day01")
  |> String.trim()
  |> String.split("\n\n", trim: True)

[a, b, c | _] =
  for b <- blocks do
    for line <- String.split(b, "\n", trim: True) do
      {x, _} = Integer.parse(line)
      x
    end
    |> Enum.sum()
  end
  |> Enum.sort(:desc)

IO.puts("part 1: #{a}")
IO.puts("part 2: #{a + b + c}")
