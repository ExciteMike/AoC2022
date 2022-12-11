defmodule Day05 do
  def read_state(s) do
    for row <- String.split(s, ~r/\R/), reduce: [[], [], [], [], [], [], [], [], []] do
      tails ->
        heads =
          for chunk <- String.graphemes(row) |> Enum.chunk_every(4) do
            Enum.filter(chunk, fn s -> String.match?(s, ~r/[[:alpha:]]/) end)
          end

        for {head, tail} <- Enum.zip(heads, tails), do: head ++ tail
    end
  end

  def run_procedure(state, rev, s) do
    for line <- String.split(s, ~r/\R/), reduce: state do
      state ->
        # parse values
        [_, {count, _}, _, {from, _}, _, {to, _}] =
          String.split(line)
          |> Enum.map(&Integer.parse/1)

        # correct indices
        from = from - 1
        to = to - 1

        # chunk grabbed by crane
        to_move =
          Enum.at(state, from)
          |> Enum.take(-count)

        # possibly reversed
        to_move =
          if rev do
            Enum.reverse(to_move)
          else
            to_move
          end

        # truncate from, concat onto to
        List.update_at(state, from, fn l -> Enum.slice(l, 0..(-count - 1)//1) end)
        |> List.update_at(to, &(&1 ++ to_move))
    end
    |> Enum.map(&List.last/1)
  end
end

[state, procedure] =
  File.read!("puzzle_input/day05")
  |> String.trim_trailing()
  |> String.split(~r/\R\R/, trim: True)

state = Day05.read_state(state)

p1 = Day05.run_procedure(state, true, procedure)
p2 = Day05.run_procedure(state, false, procedure)

IO.puts("part 1: #{p1}")
IO.puts("part 2: #{p2}")
