# julia src/day14/main.jl
EXAMPLE = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"
ROCK = 0
SAND = 1

const Coord = Tuple{Int64,Int64}
const State = Dict{Coord,Int64}

function build_map()::State
    blocked = State()
    for line in readlines("puzzle_input/day14")
        coords = split(line, " -> ") .|>
                 (raw_coord -> begin
            split(raw_coord, ",") .|>
            (s -> parse(Int64, s))
        end)
        for ((x1, y1), (x2, y2)) in zip(coords, coords[2:end])
            x1, x2 = minmax(x1, x2)
            y1, y2 = minmax(y1, y2)
            for coord in Iterators.product(x1:x2, y1:y2)
                blocked[coord] = ROCK
            end
        end
    end
    blocked
end

function draw(blocked)
    left, right, top, bottom = 500, 500, 0, 0

    for (x, y) in keys(blocked)
        left = min(left, x)
        right = max(right, x)
        top = min(top, y)
        bottom = max(bottom, y)
    end

    left -= 1
    right += 1
    bottom += 1

    for y in top:bottom
        for x in left:right
            if haskey(blocked, (x, y))
                type = blocked[(x, y)]
                if type == ROCK
                    print("%%")
                else
                    print("::")
                end
            else
                print("  ")
            end
        end
        println()
    end
end

function find(f, xs)
    for x in xs
        if f(x)
            return x
        end
    end
    nothing
end

function drop_sand(blocked::State, has_floor::Bool, bottom::Int64)::Union{Coord,Nothing}
    x = 500
    y = 0
    if haskey(blocked, (x, y))
        return nothing
    end
    while true
        # return once void reached, or come to rest on floor
        if y >= bottom
            return has_floor ? (x, y) : nothing
        end
        # fall one step
        next = find([x, x - 1, x + 1]) do x
            !haskey(blocked, (x, y + 1))
        end
        if nothing === next
            # found where it came to rest!
            return (x, y)
        end
        x = next
        y = y + 1
    end
end

function play(blocked, has_floor, bottom)
    while true
        coord = drop_sand(blocked, has_floor, bottom)
        if nothing === coord
            break
        end
        blocked[coord] = SAND
    end
end

blocked = build_map()

lowest_rock = 0
for (_, y) in keys(blocked)
    global lowest_rock = max(lowest_rock, y)
end

rock_count = length(blocked)

play(blocked, false, lowest_rock)
#draw(blocked)
println("part 1: ", length(blocked) - rock_count) # 913

play(blocked, true, lowest_rock + 1)
#draw(blocked)
println("part 2: ", length(blocked) - rock_count) # 30762