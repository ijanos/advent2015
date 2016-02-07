import fileinput
from collections import defaultdict


lines = []
for line in fileinput.input():
    lines.append(line.strip())
    
starter = lines[-1]

rules = defaultdict(list)
for line in lines[:-2]:
    a, b = line.split(" => ")
    rules[a].append(b)


result = set()

for (i, c) in enumerate(starter):
    if c in rules:
        for new in rules[c]:
                result.add(starter[:i] + new + starter[i+1:])
    
    if c + starter[i+1:i+2] in rules:
        for new in rules[c + starter[i+1:i+2]]:
                result.add(starter[:i] + new + starter[i+2:])
    
print("Answer:", len(result))