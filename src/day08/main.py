def main(input):
    heights = [[int(c) for c in row] for row in input.rstrip().splitlines()]
    grid_width = len(heights[0])
    grid_height = len(heights)

    def check(dx, dy, x, y):
        h = heights[y][x]
        x += dx
        y += dy
        while (0 <= x < grid_height) and (0 <= y < grid_width):
            if heights[y][x] >= h:
                return 0
            x += dx
            y += dy
        return 1

    def vis(x, y):
        if x == 0:
            return 1
        elif x == grid_width-1:
            return 1
        if y == 0:
            return 1
        if y == grid_height-1:
            return 1
        if check(0, -1, x, y):
            return 1
        if check(0, 1, x, y):
            return 1
        if check(1, 0, x, y):
            return 1
        if check(-1, 0, x, y):
            return 1
        return 0

    def p2_view_dist(dx, dy, x, y):
        h = heights[y][x]
        x += dx
        y += dy
        score = 0
        while (0 <= x < grid_height) and (0 <= y < grid_width):
            score += 1
            h2 = heights[y][x]
            if h2 >= h:
                break
            x += dx
            y += dy
        return score

    def p2_score(x, y):
        return p2_view_dist(0, -1, x, y) * p2_view_dist(0, 1, x, y) * p2_view_dist(1, 0, x, y) * p2_view_dist(-1, 0, x, y)

    visibility = [[vis(x, y) for x in range(grid_width)]
                  for y in range(grid_height)]
    p2_scores = [[p2_score(x, y) for x in range(grid_width)]
                  for y in range(grid_height)]
    p1 = sum(sum(x) for x in visibility)
    p2 = max(max(x) for x in p2_scores)
    print(f"part 1: {p1}")
    print(f"part 2: {p2}")
