lines =
  File.read!("puzzle_input/day02")
  |> String.trim()
  |> String.split("\n", trim: True)

p1 =
  for line <- lines do
    case line do
      "A X" -> 3 + 1
      "A Y" -> 6 + 2
      "A Z" -> 0 + 3
      "B X" -> 0 + 1
      "B Y" -> 3 + 2
      "B Z" -> 6 + 3
      "C X" -> 6 + 1
      "C Y" -> 0 + 2
      "C Z" -> 3 + 3
    end
  end
  |> Enum.sum()

p2 =
  for line <- lines do
    case line do
      "A X" -> 0 + 3
      "A Y" -> 3 + 1
      "A Z" -> 6 + 2
      "B X" -> 0 + 1
      "B Y" -> 3 + 2
      "B Z" -> 6 + 3
      "C X" -> 0 + 2
      "C Y" -> 3 + 3
      "C Z" -> 6 + 1
    end
  end
  |> Enum.sum()

IO.puts("part 1: #{p1}")
IO.puts("part 2: #{p2}")
