blocks =
  File.read!("puzzle_input/day01")
  |> String.trim()
  |> String.split(~r/\R\R/, trim: True)

[a, b, c | _] =
  for b <- blocks do
    for line <- String.split(b, ~r/\R/, trim: True) do
      {x, _} = Integer.parse(line)
      x
    end
    |> Enum.sum()
  end
  |> Enum.sort(:desc)

IO.puts("part 1: #{a}")
IO.puts("part 2: #{a + b + c}")
