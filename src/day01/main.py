def main(input):
    # day-specific code begin
    elves = sorted((sum(int(line)for line in block.split())
                    for block in input.split('\n\n')), reverse=True)
    print(f'part 1: {elves[0]}')
    print(f'part 2: {sum(elves[:3])}')
