import fileinput

program = []
for line in fileinput.input():
    command, param = line.strip().split(maxsplit=1)
    if command in ["jio", "jie"]:
        reg, value = param.split(', ')
        program.append((command, reg, int(value)))
    elif command == 'jmp':
        program.append((command, int(param)))
    else:
        program.append((command, param))

# Part 1
reg = {'a':0, 'b': 0} 

# Part 2
# reg = {'a':1, 'b': 0}

pc = 0
while True:
    if pc >= len(program):
        break
    cmd = program[pc][0]
    if cmd == 'hlf':
        reg[program[pc][1]] //= 2
        pc += 1 
    elif cmd == 'tpl':
        reg[program[pc][1]] *= 3
        pc += 1  
    elif cmd == 'inc':
        reg[program[pc][1]] += 1
        pc += 1          
    elif cmd == 'jmp':
        pc += program[pc][1]
    elif cmd == 'jie':
        if reg[program[pc][1]] % 2 == 0:
            pc += program[pc][2]
        else:
            pc += 1
    elif cmd == 'jio':
        if reg[program[pc][1]] == 1:
            pc += program[pc][2]
        else:
            pc += 1

print("Finished", reg)