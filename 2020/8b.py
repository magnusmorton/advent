import sys


prog = []
for line in sys.stdin:
    split = line.split()
    prog.append((split[0], int(split[1])))

visited = [0] * len(prog)
flipped = [0] * len(prog)

acc = 0
pc = 0
print(f"{len(prog)} instructions")


while (pc < len(prog)):
    acc = 0
    pc = 0
    changed = False
    visited = [0] * len(prog)
    while(pc < len(prog) and visited[pc] == 0):
        visited[pc] = 1
        ins, arg = prog[pc]
        if ins == "nop":
            if flipped[pc] == 0 and not changed:
                flipped[pc] = 1
                changed = True
                pc += arg
            else:
                pc += 1
        elif ins == "acc":
            acc += arg
            pc += 1
        elif ins == "jmp":
            if flipped[pc] == 0 and not changed:
                flipped[pc] = 1
                changed = True
                pc += 1
            else:
                pc += arg
print(acc)
