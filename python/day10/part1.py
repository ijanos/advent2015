#!/usr/bin/env python3

from itertools import chain

def lookandsay(n):
    acc = [(1,n[0])]
    for c in n[1:]:
        count, num = acc[-1]
        if num == c:
            acc[-1] = (count+1, num)
        else:
            acc += [(1, c)]
    return "".join(["".join((str(x),y)) for (x,y) in acc])

seed = "3113322113"
for i in range(50):
    seed = lookandsay(seed)
    if i == 39:
        print("Answer 1:", len(seed))

print("Answer 2:", len(seed))    