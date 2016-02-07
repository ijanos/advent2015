#!/usr/bin/env python3

import fileinput

nice = 0

for line in fileinput.input():
    name = line.strip()
    
    l = len(name)
    i = 0
    pairs = {}
    repeat = False
    paired = False    
    
    while i < l - 1:
        pair = name[i:i+2]
        if pair in pairs:
           j = pairs.get(pair)
           if not j == i - 1:
              paired = True
        else:
           pairs[pair] = i
        triplet = name[i:i+3]
        if len(triplet) == 3 and triplet[0] == triplet[2]:
            repeat = True 
        i += 1
        
    if repeat and paired:
        nice += 1

print(nice)