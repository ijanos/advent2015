from itertools import count

for house in count(700000):
    gifts = (house + sum([x for x in range(1, (house // 2) + 1) if house % x == 0])) * 10
    if gifts >= 34000000:
        print("Answer:", house)
        break
