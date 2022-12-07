defmodule Day07 do
  def process_line(line, {cd, sizes}) do
    case String.split(line, ~r/\s+/, trim: true) do
      ["$", "cd", ".."] ->
        {tl(cd), sizes}

      ["$", "cd", "/"] ->
        {[], sizes}

      ["$", "cd", name] ->
        {[name | cd], sizes}

      ["$", "ls"] ->
        {cd, sizes}

      ["dir", name] ->
        {cd, Map.put(sizes, [name | cd], 0)}

      [s, _] ->
        {bytes, _} = Integer.parse(s)
        {cd, Day07.updateSizes(cd, bytes, sizes)}
    end
  end

  def updateSizes(path, bytes, sizes) do
    if Enum.empty?(path) do
      Map.update!(sizes, path, &(&1 + bytes))
    else
      updateSizes(tl(path), bytes, Map.update!(sizes, path, &(&1 + bytes)))
    end
  end
end

{_, data} =
  File.read!("puzzle_input/day07")
  |> String.trim_trailing()
  |> String.split(~r/\R+/, trim: True)
  |> Enum.reduce({[], %{[] => 0}}, &Day07.process_line/2)

p1 =
  Map.values(data)
  |> Enum.filter(&(&1 < 100_000))
  |> Enum.sum()

IO.puts("part 1: #{p1}")

toFree = 30_000_000 - 70_000_000 + Map.get(data, [])

p2 =
  Map.values(data)
  |> Enum.filter(&(&1 >= toFree))
  |> Enum.min()

IO.puts("part 2: #{p2}")
