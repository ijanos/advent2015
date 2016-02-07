#!/bin/env python3

import fileinput

floor = 0
pos = 0

for line in fileinput.input():
    for c in line:
        pos += 1
        if c == "(":
            floor +=1
        if c == ")":
            floor -= 1
            if floor == -1:
                print(pos)
                exit()