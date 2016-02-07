import fileinput
from itertools import combinations


containers = []
result = 0

for line in fileinput.input():
    containers.append(int(line.strip()))

for r in range(2, len(containers)+1):
    for order in combinations(containers, r):
        if sum(order) == 150:
            result += 1

print("Answer:", result)
