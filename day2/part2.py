#!/bin/env python3 

import fileinput
from functools import reduce
from operator import mul

result = 0
 
for line in fileinput.input():
    dimensions = [int(n) for n in line.strip().split("x")]
    dimensions.sort()
    result += (2 * dimensions[0]) + (2 * dimensions[1]) + reduce(mul, dimensions)
print(result)