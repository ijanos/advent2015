#!/usr/bin/env python3

import fileinput

grid = {}

def init():
    for i in range(0,1000):
        for j in range(0,1000):
            grid[(i,j)] = False
    return grid
   

def switch(x1,y1,x2,y2,state):
    for x in range(x1,x2+1):
        for y in range(y1,y2+1):
            if state == "on":
                grid[(x,y)] = True
            if state == "off":
                grid[(x,y)] = False
            if state == "toggle":
                grid[(x,y)] = not grid[(x,y)]


def getxy(start, stop):
    x1,y1 = start.split(',')
    x2,y2 = stop.split(',')
    return (int(x1), int(y1), int(x2), int(y2))        

init()               
for line in fileinput.input():
    line = line.strip().split()
    if len(line) == 5:
        x1,y1,x2,y2 = getxy(line[2], line[4])
        switch(x1,y1,x2,y2,line[1])
    if len(line) == 4:
        x1,y1,x2,y2 = getxy(line[1],line[3])
        switch(x1,y1,x2,y2,'toggle')

print(len([x for x in grid.values() if x]))            