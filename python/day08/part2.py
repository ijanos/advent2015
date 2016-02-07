#!/usr/bin/env python3

import fileinput
from re import escape

answer = 0

for line in fileinput.input():
    line = line.strip()
    answer += len(escape(line)) + 2
    answer -= len(line)

print("Answer: ", answer)
