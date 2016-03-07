import string
from itertools import groupby

abc = string.ascii_lowercase

def increment(pw):
    if pw[-1] == "z":
        if len(pw) == 1:
            return "a"
        return increment(pw[:-1]) + "a"
    return pw[:-1] + abc[abc.find(pw[-1]) + 1]


def valid(pw):
    if 'i' in pw or 'o' in pw or 'l' in pw:
        return False

    triplet = False
    for i in range(len(pw) - 2):
        if pw[i:i+3] in abc:
            triplet = True
            break
    if not triplet:
        return False

    doubles = 0
    for c,l in groupby(pw):
        if len(list(l)) > 1:
            doubles += 1
        if doubles == 2:
            break
    if doubles < 2:
        return False
    return True

pw = "vzbxkghb"

while not valid(pw):
    pw = increment(pw)

print("Answer 1:", pw)

pw = increment(pw)
while not valid(pw):
    pw = increment(pw)

print("Answer 2:", pw)
