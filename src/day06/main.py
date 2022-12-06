def day6(input, n):
    test = ''
    for i, c in enumerate(input):
        test += c
        if len(test) == n:
            if n == len(set(test)):
                return i+1
            test = test[1:]

def main(input):
    print(f"part 1: {day6(input, 4)}")
    print(f"part 2: {day6(input, 14)}")