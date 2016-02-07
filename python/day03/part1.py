import fileinput

visited = set()
current = (0,0)

update = {
    '>': lambda xy: (xy[0] + 1, xy[1]    ),
    '<': lambda xy: (xy[0] - 1, xy[1]    ),
    '^': lambda xy: (xy[0]    , xy[1] + 1),
    'v': lambda xy: (xy[0]    , xy[1] - 1)
}

for line in fileinput.input():
    for c in line.strip():
       visited.add(current)
       current = update[c](current)
    print(len(visited))

