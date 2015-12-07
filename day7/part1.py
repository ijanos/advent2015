#!/usr/bin/env python3

import fileinput

rules = {}
cache = {}

do = {
    "AND":    lambda ab: ab[0] &  ab[1],
    "OR":     lambda ab: ab[0] |  ab[1],
    "LSHIFT": lambda an: an[0] << an[1],
    "RSHIFT": lambda an: an[0] >> an[1],
    "NOT":    lambda  a: ~a
}


def solve(n):
    if n in cache:
        return cache[n]
    if n.isdigit():
        return int(n)
    else:
        result = solver(n)
        cache[n] = int(result)
        return result


def solver(n):
    command = rules[n]
    command = command.split()
    if len(command) == 1:
        return solve(command[0])
    if len(command) == 2:
        return do[command[0]](solve(command[1]))
    if len(command) == 3:
        return do[command[1]]((solve(command[0]), solve(command[2])))


for line in fileinput.input():
    line = line.strip()
    command, wire = line.split(' -> ')
    rules[wire] = command

print("result: ", solve('a'))
