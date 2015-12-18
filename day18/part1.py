import fileinput


board = {}

for x, row in enumerate(fileinput.input()):
    for y, cell in enumerate(row.strip()):
        board[(x,y)] = 1 if cell == "#" else 0

steps = 0      
while steps < 100:
    newboard = {} 
    for (x,y), state in board.items():
        neighbors = 0
        for neighbor in [(x-1, y-1), (x, y-1), (x+1, y-1),
                         (x-1, y  ),           (x+1, y  ),
                         (x-1, y+1), (x, y+1), (x+1, y+1)]:
            if neighbor in board and board[neighbor] == 1:
                neighbors += 1
        
        if state == 1 and neighbors in [2, 3]:
            newboard[(x,y)] = 1
        elif state == 0 and neighbors == 3:
            newboard[(x,y)] = 1
        else:
            newboard[(x,y)] = 0
            
    board = newboard
    steps += 1

print("Answer:", sum(board.values()))