from typing import Callable, List, Self


class Monkey:
    items: list[int]
    op: Callable[[int], int]

    def __init__(self, items, op, test, pass_dst, fail_dst):
        self.items = items
        self.op = op
        self.test = test
        self.pass_dst = pass_dst
        self.fail_dst = fail_dst
        self.count = 0

    def clone(self) -> Self:
        return Monkey(self.items[:], self.op, self.test, self.pass_dst, self.fail_dst)

    def take_turn(self, ms: List[Self], div3):
        self.count += len(self.items)
        for item in self.items:
            item = self.op(item)
            item = item//3 if div3 else item % MODULO
            dst = self.pass_dst if (item % self.test) == 0 else self.fail_dst
            ms[dst].items.append(item)
        self.items = []


MODULO = 2 * 3 * 5 * 7 * 11 * 13 * 17 * 19 * 23


def play_keep_away(ms: List[Monkey], rounds, div3) -> int:
    for _ in range(rounds):
        for m in ms:
            m.take_turn(ms, div3)
    [a, b] = sorted(m.count for m in ms)[-2:]
    return a*b


def parse_monkey(mcode: str) -> Monkey:
    lines = mcode.splitlines()
    match list(lines[2][23:].split()):
        case ["*", "old"]:
            def op(x): return x*x
        case ["*", s]:
            n = int(s)
            def op(x): return x * n
        case ["+", s]:
            n = int(s)
            def op(x): return x + n
        case _:
            def op(x): return x
    return Monkey(
        [int(x) for x in lines[1][18:].split(", ")],
        op,
        int(lines[3][21:]),
        int(lines[4][29:]),
        int(lines[5][30:])
    )


def main(input: str):
    ms = [parse_monkey(s) for s in input.split('\n\n')]
    print(f"""part 1: {
            play_keep_away([m.clone() for m in ms], 20, True)
            }\npart 2: {
            play_keep_away(ms, 10000, False)}""")
