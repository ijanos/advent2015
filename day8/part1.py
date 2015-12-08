#!/usr/bin/env python3

import fileinput

answer = 0

for line in fileinput.input():
    line = line.strip()
    answer += len(line)
    answer -= len(eval(line)) # this is crazy, I know

print("Answer: ", answer)
