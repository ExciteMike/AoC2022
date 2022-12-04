def priority(c):
    adjust = -38 if c.isupper() else -96
    return ord(c) + adjust


def score_p1(s):
    i = len(s)//2
    return next(iter(set(s[:i]).intersection(s[i:])))


def main(input):
    sacks = [[priority(c) for c in line] for line in input.splitlines()]
    p1 = sum(score_p1(s) for s in sacks)
    p2 = 0
    for i in range(0, len(sacks), 3):
        a, b, c = sacks[i:i+3]
        p2 += next(iter(set(a).intersection(b).intersection(c)))
    print(f'part 1: {p1}')
    print(f'part 2: {p2}')
