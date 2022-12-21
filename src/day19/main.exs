defmodule Day19 do
  @moduledoc "AoC2022 Day19 functions"
  alias ElixirDS.Deque

  def play(bp, minutes) do
    deq = Deque.from_list([{[1, 0, 0, 0], [0, 0, 0, 0], [true, true, true, true], 1}])
    caps = Enum.zip(bp) |> Enum.map(fn {a, b, c, d} -> Enum.max([a, b, c, d]) end)
    play(bp, minutes, caps, deq, 0)
  end

  defp play(bp, minutes, caps, deq, best) do
    # breadth-first so that we can easily tell when it's falling behind
    case Deque.pop_front(deq) do
      :error ->
        best

      {{bots, resources, allow, time}, deq} ->
        [_, _, _, geodes] = resources
        best = max(best, geodes)

        if time > minutes do
          play(bp, minutes, caps, deq, best)
        else
          [_, _, _, bots_geodes] = bots
          # PRUNING - falling behind the rest of the BFS
          if geodes + bots_geodes + 1 < best do
            play(bp, minutes, caps, deq, best)
          else
            can_afford = Enum.map(bp, fn cost -> can_afford(cost, resources) end)

            # PRUNING - if we skipped building something we could, no point in considering building it next time
            allow_after_skip = Enum.map(can_afford, &(not &1))

            {stop, deq} =
              Enum.zip([caps, bots, allow, bp, can_afford])
              |> Enum.with_index()
              |> Enum.reverse()
              |> Enum.flat_map(fn {{cap, bot_count, allow, cost, can_afford}, i} ->
                full = i !== 3 and bot_count >= cap

                if !full and allow and can_afford do
                  [{i, build(bots, cost, resources, time, i)}]
                else
                  []
                end
              end)
              # PRUNING - stop here is so that if we built a geode bot, don't consider other bots or skipping
              |> Enum.reduce({false, deq}, fn {i, item}, {stop, deq} ->
                if stop do
                  {true, deq}
                else
                  {i == 3, Deque.push_back(deq, item)}
                end
              end)

            deq =
              if stop do
                deq
              else
                resources = Enum.zip(resources, bots) |> Enum.map(fn {r, b} -> r + b end)
                Deque.push_back(deq, {bots, resources, allow_after_skip, time + 1})
              end

            play(bp, minutes, caps, deq, best)
          end
        end
    end
  end

  defp build(bot_counts, cost, resources, time, i) do
    resources = Enum.zip([resources, cost, bot_counts]) |> Enum.map(fn {r, c, b} -> r - c + b end)
    bot_counts = List.update_at(bot_counts, i, &(&1 + 1))
    {bot_counts, resources, [true, true, true, true], time + 1}
  end

  defp can_afford(costs, resources) do
    Enum.zip(costs, resources)
    |> Enum.all?(fn {a, b} -> a <= b end)
  end
end

bps =
  File.read!("puzzle_input/day19")
  |> String.trim_trailing()
  |> String.split(~r/[^0-9]+/, trim: True)
  |> Enum.filter(fn s -> s !== "" end)
  |> Enum.map(&String.to_integer/1)
  |> Enum.chunk_every(7)
  |> Enum.map(fn [_, x1, x2, x3, x4, x5, x6] ->
    [[x1, 0, 0, 0], [x2, 0, 0, 0], [x3, x4, 0, 0], [x5, 0, x6, 0]]
  end)

p1 =
  Enum.map(bps, fn bp ->
    Day19.play(bp, 24)
  end)
  |> Enum.with_index(1)
  |> Enum.map(fn {x, i} -> x * i end)
  |> Enum.sum()

p2 = Enum.take(bps, 3) |> Enum.map(fn bp -> Day19.play(bp, 32) end) |> Enum.product()

# 1395, 2700
IO.puts("part 1: #{p1}\npart 2: #{p2}")
