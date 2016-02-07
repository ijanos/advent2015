from itertools import count

houses = []
for x in range(5000000):
    houses.append(0)
    
for i in range(1, len(houses)):
    for j in range(i, min(len(houses), 50 * i + 1), i):
        houses[j] += i * 11


for n,gifts in enumerate(houses):
    if gifts >= 34000000:
        print('Answer:', n)
        exit()  