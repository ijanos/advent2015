import fileinput
from collections import defaultdict


deers = []


def distance(time, speed, fly, rest):
    return time // (fly+rest) * speed * fly + speed * min(fly, time % (fly+rest))


for line in fileinput.input():
    line = line.strip().split()
    speed, fly, rest = map(int, [line[3], line[6], line[13]])
    time = 2503
    pos = []
    for t in range(time):
        pos.append(distance(t+1, speed, fly, rest))
    deers.append(pos)

points = defaultdict(int)

for actual in zip(*deers):
    winner = max(actual)
    for i in range(len(actual)):
        if actual[i] == winner:
            points[i] += 1

print("Answer:", max(points.values()))

