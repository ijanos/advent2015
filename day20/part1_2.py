from itertools import count

houses = []
for x in range(1000000):
    houses.append(0)
    
for i in range(1,len(houses)):
    for j in range(i,len(houses),i):
        houses[j] += i * 10
        if houses[j] >= 34000000:
            print(j)
            exit()  