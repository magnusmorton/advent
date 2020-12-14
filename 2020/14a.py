import sys

mem = {}
for line in sys.stdin:
    ex = line.split(" = ")
    command = ex[0]
    rvalue = ex[1].rstrip()
    if command == "mask":
        or_mask = int(rvalue.replace('X', '0'), 2)
        and_mask = int(rvalue.replace('X', '1'), 2)
    else:
        loc = int(command.split('[')[1][:-1])
        val = int(rvalue)
        mem[loc] = (or_mask | val) & and_mask

print(sum(mem.values()))
