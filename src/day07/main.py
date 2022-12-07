def parents_of(path):
    while path != '/':
        path = path.rsplit('/', 1)[0] or '/'
        yield path


def main(input):
    curpath = '/'
    dirs = {'/': 0}
    for line in input.splitlines():
        match line.split():
            case['$', 'ls']: pass
            case['$', 'cd', '..']:
                curpath = curpath.rsplit('/', 1)[0] or '/'
            case['$', 'cd', '/']:
                curpath = '/'
            case['$', 'cd', s]:
                curpath = f'{curpath.rstrip("/")}/{s}'
            case['dir', s]:
                path = f'{curpath.rstrip("/")}/{s}'
                if path not in dirs:
                    dirs[path] = 0
            case[size, fname]:
                size = int(size)
                path = f'{curpath.rstrip("/")}/{fname}'
                for parent in parents_of(path):
                    dirs[parent] += size

    p1 = sum(size for _, size in dirs.items() if size < 100000)
    print(f"part 1: {p1}")

    to_free = 30000000 - 70000000 + dirs['/']
    p2 = min(size for size in dirs.values() if size >= to_free)
    print(f"part 2: {p2}")
