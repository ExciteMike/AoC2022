defmodule Day11 do
  @modulo 2 * 3 * 5 * 7 * 11 * 13 * 17 * 19 * 23

  def square(x) do
    x * x
  end

  defmodule M do
    defstruct items: [], op: &Day11.square/1, test: 1, pass: 0, fail: 0, count: 0

    def receive(m, x) do
      Map.update!(m, :items, fn xs -> xs ++ [x] end)
    end

    def inc(m) do
      Map.update!(m, :count, &(&1 + 1))
    end

    def clear(m) do
      Map.put(m, :items, [])
    end
  end

  def parse(s) do
    [_, items, op, test, pass, fail] = String.split(s, ~r/\R/, trim: True)

    items =
      String.slice(items, 18..-1)
      |> String.split()
      |> Enum.map(&Day11.int/1)

    op =
      case String.split(String.slice(op, 23..-1)) do
        ["*", "old"] ->
          &Day11.square/1

        ["*", s] ->
          n = int(s)
          fn x -> x * n end

        ["+", s] ->
          n = int(s)
          fn x -> x + n end
      end

    test = int(String.slice(test, 21..-1))
    pass = int(String.slice(pass, 29..-1))
    fail = int(String.slice(fail, 30..-1))

    %M{items: items, op: op, test: test, pass: pass, fail: fail, count: 0}
  end

  def play(ms, rounds, div3) do
    for _ <- 1..rounds, reduce: ms do
      ms ->
        for i <- Map.keys(ms), reduce: ms do
          ms -> Day11.take_turn(ms, i, div3)
        end
    end
    |> Enum.map(fn {_, m} -> m.count end)
    |> Enum.concat([0])
    |> Enum.sort()
    |> Enum.reverse()
    |> Enum.take(2)
    |> Enum.product()
  end

  def int(s) do
    {n, _} = Integer.parse(s)
    n
  end

  def take_turn(ms, i, div3) do
    m = Map.get(ms, i)

    for x <- m.items, reduce: ms do
      ms ->
        x = m.op.(x)

        x =
          if div3 do
            div(x, 3)
          else
            rem(x, @modulo)
          end

        dst =
          case rem(x, m.test) do
            0 -> m.pass
            _ -> m.fail
          end

        Map.update!(ms, dst, fn m -> M.receive(m, x) end)
        |> Map.update!(i, &M.inc/1)
        |> Map.update!(i, &M.clear/1)
    end
  end
end

ms =
  File.read!("puzzle_input/day11")
  |> String.trim_trailing()
  |> String.split(~r/\R\R/, trim: True)
  |> Stream.map(&Day11.parse/1)
  |> Stream.with_index()
  |> Map.new(fn {v, i} -> {i, v} end)

IO.puts("part 1: #{Day11.play(ms, 20, True)}")
IO.puts("part 2: #{Day11.play(ms, 10000, False)}")
