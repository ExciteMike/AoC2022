import re

EXAMPLE = """$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
"""

CMD = re.compile('\$')
UP = re.compile('\$ cd \.\.')
TOP = re.compile('\$ cd /')
DOWN = re.compile('\$ cd (.*)')
LS = re.compile('\$ ls')
DIR = re.compile('dir (.*)')
SIZE = re.compile('(\d+) (.*)')


def cd_up(path):
    x = path.rsplit('/', 1)[0]
    if not x:
        x = '/'
    return x


def parents_of(path):
    while path != '/':
        path = cd_up(path)
        yield path


def main(input):
    curpath = '/'
    dirs = {'/': 0}
    files = []
    for line in input.splitlines():
        if LS.match(line):
            continue

        # cd up a level
        if UP.match(line):
            curpath = cd_up(curpath)
            continue

        # cd to top
        if TOP.match(line):
            curpath = '/'
            continue

        # cd down
        m = DOWN.match(line)
        if m:
            curpath = f'{curpath.rstrip("/")}/{m.group(1)}'
            continue

        # dir
        m = DIR.match(line)
        if m:
            path = f'{curpath.rstrip("/")}/{m.group(1)}'
            if path not in dirs:
                dirs[path] = 0
            continue

        # size
        m = SIZE.match(line)
        if m:
            size = int(m.group(1))
            path = f'{curpath.rstrip("/")}/{m.group(2)}'
            files.append((path, size))
            for parent in parents_of(path):
                dirs[parent] += size
            continue
        print(f'unhandled: "{line}"')
    p1 = sum(size for path, size in dirs.items() if size < 100000)
    print(f"part 1: {p1}")

    max_space = 70000000
    needed = 30000000
    unused = max_space - dirs['/']
    to_free = needed - unused
    p2 = min(size for size in dirs.values() if size >= to_free)
    print(f"part 2: {p2}")
