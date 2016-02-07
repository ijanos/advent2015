import fileinput
from random import shuffle
from functools import reduce
from operator import mul
from sys import maxsize

gifts = []

for line in fileinput.input():
    gifts.append(int(line.strip()))

third = sum(gifts) / 3

def getThird(gifts, third):
    while True:
        s = 0
        group = []
        shuffle(gifts)
        for n in gifts:
            if s > third:
                break
            if s < third:
                group.append(n)
                s += n
            if s == third:
                return group, gifts[len(group):]

min = 30
minqe = maxsize

while True:
    g = {}
    g[1], rest = getThird(gifts, third)
    g[2], g[3] = getThird(rest, third)
    for i in [1,2,3]:
        qe = reduce(mul, g[i])
        if len(g[i]) <= min and qe < minqe:
            min = len(g[i])
            minqe = qe
            print(len(g[i]), i, qe, g)
