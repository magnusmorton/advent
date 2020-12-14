import sys
import itertools

mem = {}
for line in sys.stdin:
    ex = line.split(" = ")
    command = ex[0]
    rvalue = ex[1].rstrip()
    if command == "mask":
        mask = int(rvalue.replace('X', '0'), 2)
        xs = [35-i for i, ch in enumerate(rvalue) if ch == 'X']
    else:
        loc = int(command.split('[')[1][:-1])
        val = int(rvalue)
        loc = loc | mask
        bitsets = itertools.product(range(2), repeat=len(xs))
        for bits in bitsets:
            nloc = loc
            for bit, i in zip(bits, xs):
                nloc = (nloc & ~(1 << i)) | (bit << i)
            mem[nloc] = val

print(sum(mem.values()))
