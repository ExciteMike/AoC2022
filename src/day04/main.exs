lines =
  File.read!("puzzle_input/day04")
  |> String.trim()
  |> String.split(~r/\R/, trim: True)

ranges =
  for line <- lines do
    for half <- String.split(line, ",") do
      for x <- String.split(half, "-") do
        {x, _} = Integer.parse(x)
        x
      end
    end
  end

overlaps =
  for [[a_lo, a_hi], [b_lo, b_hi]] <- ranges,
      max(a_lo, b_lo) <= min(a_hi, b_hi),
      do: (a_lo <= b_lo && a_hi >= b_hi) || (a_lo >= b_lo && a_hi <= b_hi)

IO.puts("part 1: #{Enum.count(overlaps, & &1)}")
IO.puts("part 2: #{Enum.count(overlaps)}")
