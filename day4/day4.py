#!/usr/bin/env python3

from hashlib import md5

secret = "bgvyzdsv"
i = 1
done = [False, False]

while True:
    key = secret + str(i)
    if not done[0] and md5(key.encode()).hexdigest()[0:5] == "00000":
        print("Part 1: " + str(i))
        done[0] = True
    if not done[1] and md5(key.encode()).hexdigest()[0:6] == "000000":
        print("Part 2: " + str(i))
        done[1] = True
    if done == [True, True]:
        exit()
    i += 1
