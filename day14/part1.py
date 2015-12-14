import fileinput

winner = 0

for line in fileinput.input():
    line = line.strip().split()
    speed, fly, rest = map(int, [line[3], line[6], line[13]])
    time = 2503
    distance = time // (fly + rest) * speed * fly + \
                speed * min(fly, time % (fly+rest))
    winner = max(winner, distance)

print("Answer:", winner)
