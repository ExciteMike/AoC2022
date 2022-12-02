score_p1 = {
    'A X': 3 + 1,
    "A Y": 6 + 2,
    "A Z": 0 + 3,
    "B X": 0 + 1,
    "B Y": 3 + 2,
    "B Z": 6 + 3,
    "C X": 6 + 1,
    "C Y": 0 + 2,
    "C Z": 3 + 3,
}
score_p2 = {
    "C Y": 0 + 2,
    "A X": 0 + 3,
    "A Y": 3 + 1,
    "A Z": 6 + 2,
    "B X": 0 + 1,
    "B Y": 3 + 2,
    "B Z": 6 + 3,
    "C X": 0 + 2,
    "C Y": 3 + 3,
    "C Z": 6 + 1,
}


def main(input):
    print(f'part 1: {sum(score_p1[line]for line in input.splitlines())}')
    print(f'part 2: {sum(score_p2[line]for line in input.splitlines())}')
