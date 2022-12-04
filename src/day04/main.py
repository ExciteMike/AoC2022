def main(input):
    p1 = 0
    p2 = 0
    for line in input.splitlines():
        ((a_lo, a_hi), (b_lo, b_hi)) = [
            [int(x) for x in half.split('-')] for half in line.split(',')]
        if max(a_lo, b_lo) <= min(a_hi, b_hi):
            p2 += 1
            if ((a_lo <= b_lo) and (a_hi >= b_hi)) or ((a_lo >= b_lo) and (a_hi <= b_hi)):
                p1 += 1
    print(f'part 1: {p1}')
    print(f'part 2: {p2}')
