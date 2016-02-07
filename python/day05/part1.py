#!/usr/bin/env python3

import fileinput

def nice(name):
    if 'ab' in name or 'cd' in name or 'pq' in name or 'xy' in name:
        return False
   
    vowels = 0
    prev = None
    contains_double = False 
   
    for c in name:
        if c in 'aeiou':
           vowels += 1
        if c == prev:
           contains_double = True
        prev = c
    if vowels > 2 and contains_double:
        return True
    return False
   
nicenames = 0
    
for line in fileinput.input():
    if nice(line.strip()):
        nicenames += 1

print(nicenames)    
       