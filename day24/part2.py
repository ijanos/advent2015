import fileinput
from random import shuffle
from functools import reduce
from operator import mul
from sys import maxsize

gifts = []

for line in fileinput.input():
    gifts.append(int(line.strip()))

goal = sum(gifts) / 4

def getSplit(gifts, goal):
    for _ in range(10000):
        s = 0
        group = []
        shuffle(gifts)
        for n in gifts:
            if s > goal:
                break
            if s < goal:
                group.append(n)
                s += n
            if s == goal:
                return group, gifts[len(group):]

min = 30
minqe = maxsize

while True:
    g = {}
    g[1], rest = getSplit(gifts, goal)
    g[2], rest = getSplit(rest, goal)
    ret = getSplit(rest, goal)
    if not ret:
        continue
    g[3], g[4] = ret
    for i in [1, 2, 3, 4]:
        qe = reduce(mul, g[i])
        if len(g[i]) <= min and qe < minqe:
            min = len(g[i])
            minqe = qe
            print('\n', len(g[i]), i, qe, g)
