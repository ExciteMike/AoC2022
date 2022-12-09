from itertools import *


def sign(x): return 1 if x > 0 else (0 if x == 0 else -1)


def follow(head, tail):
    if all(head[axis]-1 <= tail[axis] <= head[axis]+1 for axis in range(2)):
        return
    for axis in range(2):
        tail[axis] += sign(head[axis]-tail[axis])


def propagate(nodes):
    for i in range(len(nodes)-1):
        follow(*nodes[i:i+2])


def sim_rope(nodes, direction, steps, visited1, visited9):
    dx, dy = 0, 0
    match direction:
        case "U": dy = 1
        case "D": dy = -1
        case "L": dx = -1
        case "R": dx = 1
    for _ in range(steps):
        nodes[0][0] += dx
        nodes[0][1] += dy
        propagate(nodes)
        visited1.add(tuple(nodes[1]))
        visited9.add(tuple(nodes[9]))


def main(input):
    state = [[0, 0] for _ in range(10)]
    visited1 = set([tuple(state[1])])
    visited9 = set([tuple(state[9])])
    for l in input.splitlines():
        direction, distance = l.split()
        sim_rope(state, direction, int(distance), visited1, visited9)
    print(f"part 1: {len(visited1)}")
    print(f"part 2: {len(visited9)}")
