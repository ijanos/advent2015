import fileinput
from collections import defaultdict
from itertools import permutations

drama = defaultdict(dict)

for line in fileinput.input():
    line = line.strip()[:-1].split()
    person1 = line[0]
    person2 = line[10]
    if line[2] == "lose":
        happiness = int(line[3]) * -1
    else:
        happiness = int(line[3])
    drama[person1][person2] = happiness

for p in list(drama.keys()):
    drama[p]['me'] = 0
    drama['me'][p] = 0

max_drama = 0

for sitting in permutations(drama.keys()):
    drama_level = 0
    for (p1,p2) in zip(sitting,sitting[1:]+sitting[0:1]):
        drama_level += drama[p1][p2]
        drama_level += drama[p2][p1]
    max_drama = max(max_drama, drama_level)
    
print("Answer:", max_drama)