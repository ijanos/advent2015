from itertools import product
from functools import reduce
from operator import mul
import fileinput


ingredients = []

for line in fileinput.input():
    line = line.strip().split()
    ingredients.append(tuple(map(int,[line[2][:-1],
                                      line[4][:-1],
                                      line[6][:-1],
                                      line[8][:-1]])))

score = 0

for recepie in product(range(1,101), repeat=len(ingredients)):
    if sum(recepie) == 100:       
        rows = []
        for i in range(len(recepie)):
            rows.append(tuple(map(lambda x: x*recepie[i], ingredients[i])))
        score = max(score, reduce(mul, [x for x in map(sum, zip(*rows)) if x > 0]))

print("Answer:", score)
