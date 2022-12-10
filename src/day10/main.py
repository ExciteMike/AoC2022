def p1(input):
    x = 1
    xs = [1,1]
    for line in input.splitlines():
        xs.append(x)
        if line.startswith("addx"):
            x += int(line[5:])
            xs.append(x)
    result = sum((i) * xs[i] for i in range(20,221, 40))
    print(f"part 1: {result}")
    return xs

def p2(xs):
    print(f"part 2: ")
    for y in range(6):
        for x in range(40):
            sprite = xs[40*y+x]
            if 0 <= (x-sprite) <= 2:
                print('[]', end='')
            else:
                print('  ', end='')
        print()

def main(input):
    xs = p1(input)
    p2(xs)