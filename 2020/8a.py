import sys


prog = []
for line in sys.stdin:
    split = line.split()
    prog.append((split[0], int(split[1])))

visited = [0] * len(prog)

acc = 0
pc = 0

while(visited[pc] == 0):
    visited[pc] = 1
    ins, arg = prog[pc]
    if ins == "nop":
        pc += 1
    elif ins == "acc":
        acc += arg
        pc += 1
    elif ins == "jmp":
        pc += arg

print(acc)
