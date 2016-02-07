#!/bin/env python3 

import fileinput
from itertools import combinations

result = 0
 
for line in fileinput.input():
    dimensions = [int(n) for n in line.strip().split("x")]
    areas = [x * y for (x,y) in combinations(dimensions, 2)]
    result += sum([2 * a for a in areas]) + min(areas)
print(result)