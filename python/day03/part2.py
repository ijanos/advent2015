import fileinput

visited = set()
current = [(0,0), (0,0), True]

update = {
    '>': lambda xy: (xy[0] + 1, xy[1]    ),
    '<': lambda xy: (xy[0] - 1, xy[1]    ),
    '^': lambda xy: (xy[0]    , xy[1] + 1),
    'v': lambda xy: (xy[0]    , xy[1] - 1)
}

for line in fileinput.input():
    for c in line.strip():
        i = 1 if current[2] else 0
        visited.add(current[i])
        current[i] = update[c](current[i])
        current[2] = not current[2]
    print(len(visited))

