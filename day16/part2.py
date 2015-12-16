import fileinput


info = {
     "children":    "3",
     "cats":        "7",
     "samoyeds":    "2",
     "pomeranians": "3",
     "akitas":      "0",
     "vizslas":     "0",
     "goldfish":    "5",
     "trees":       "3",
     "cars":        "2",
     "perfumes":    "1"
}

winner = 0
for line in fileinput.input():
    line = line.strip().split() 
    check = 0
    for key, value in [(line[2][:-1], line[3][:1]), (line[4][:-1], line[5][:-1]), (line[6][:-1], line[7])]:
        if key in ["cats",  "trees"] and info[key] < value:
            check += 1 
        elif key in ["pomeranians",  "goldfish"] and info[key] > value:
            check += 1 
        elif key not in ["cat","pomeranians","goldfish","trees"] and info[key] == value:
            check += 1 
    if check == 3:
        print(line[1][:-1])
    
