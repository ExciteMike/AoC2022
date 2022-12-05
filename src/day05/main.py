def run(state, rev, procedure):
    state = state[:]
    for line in procedure.splitlines():
        count, stack_from, stack_to = (int(x) for x in line.split() if x.isnumeric())
        stack_from = int(stack_from) - 1
        stack_to = int(stack_to) - 1
        step = -1 if rev else 1
        grabbed = state[stack_from][:count][::step]
        state[stack_from] = state[stack_from][count:]
        state[stack_to] = grabbed + state[stack_to]
    return ''.join([x[0] for x in state])

def main(input):
    init, procedure = input.split('\n\n')
    state = ['' for _ in range(9)]
    for line in init.splitlines():
        for i in range(0, len(line), 4):
            c = line[i+1]
            if c.isalpha():
                state[i//4] += c
    print(f"part 1: {run(state, True, procedure)}")
    print(f"part 2: {run(state, False, procedure)}")
