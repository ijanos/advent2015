def rows():
    j = 1
    while True:
        for i in range(1, j):
            yield i
        j += 1

def cols():
    j = 1
    while True:
        for i in range(j, 0, -1):
            yield i
        j += 1

def part1():
    current = 20151125
    while True:
        yield current
        current = current * 252533 % 33554393

for row, col, value in zip(rows(), cols(), part1()):
    if col == 2947 and row == 3029:
        print('Answer:', value)
        break