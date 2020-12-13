import sys

pos = (0,0)
bearing = 90

dirmap = {0: (0, 1), 90: (1, 0), 180: (0, -1), 270: (-1, 0)}

for line in sys.stdin:
    ins = line[0]
    val = int(line[1:])
    if ins == 'F':
        dire = dirmap[bearing]
        offset = tuple(val * i for i in dire)
        pos = (pos[0] + offset[0], pos[1] + offset[1])
    elif ins == 'N':
        pos = (pos[0], pos[1] + val)
    elif ins == 'S':
        pos = (pos[0], pos[1] - val)
    elif ins == 'E':
        pos = (pos[0] + val, pos[1])
    elif ins == 'W':
        pos = (pos[0] - val, pos[1])
    elif ins == 'R':
        bearing = (bearing + val) % 360
    elif ins == 'L':
        bearing = (bearing - val) % 360

print(abs(pos[0]) + abs(pos[1]))
