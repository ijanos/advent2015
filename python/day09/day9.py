#!/usr/bin/env python3

import fileinput
from itertools import permutations
import sys

distance = {}
shortest = sys.maxsize
longest = 0


for line in fileinput.input():
    line = line.strip().split()

    c1 = line[0]
    c2 = line[2]
    d = line[4]

    if c1 not in distance:
        distance[c1] = {}
    if c2 not in distance:
        distance[c2] = {}

    distance[c1][c2] = int(d)
    distance[c2][c1] = int(d)

for path in permutations(distance.keys()):
    length = 0
    broke = False
    for c1, c2 in zip(path, path[1:]):
        if c2 not in distance[c1]:
            broke = True
            break
        length += distance[c1][c2]
    if not broke:
        shortest = min(shortest, length)
        longest = max(longest, length)


print("Shortest route:", shortest)
print("Longest route:", longest)
