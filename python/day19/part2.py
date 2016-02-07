import fileinput
from random import shuffle


lines = []
for line in fileinput.input():
    lines.append(line.strip())
    
starter = lines[-1]

rules = []
for line in lines[:-2]:
    rules.append(tuple(line.split(" => ")))

finished = False

steps = 0
molecule = starter    
while not finished:
    changed = False
    for (a, b) in rules:
        if b in molecule:
            molecule = molecule.replace(b, a, 1)
            steps += 1
            changed = True
            break
    if molecule == 'e':
        finished = True
    if not changed:
        shuffle(rules)
        steps = 0
        molecule = starter
        
print("Answer:", steps)