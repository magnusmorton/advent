import sys

pos = (0,0)
bearing = 90
wayp = (10,1)
dirmap = {0: (0, 1), 90: (1, 0), 180: (0, -1), 270: (-1, 0)}
cos = {0: 1, 90: 0, 180: -1, 270: 0}
sin = {0: 0, 90: 1, 180: 0, 270: -1}

for line in sys.stdin:
    ins = line[0]
    val = int(line[1:])
    if ins == 'F':
        pos = (pos[0] + wayp[0] * val, pos[1] + wayp[1] * val)
    elif ins == 'N':
        wayp = (wayp[0], wayp[1] + val)
    elif ins == 'S':
        wayp = (wayp[0], wayp[1] - val)
    elif ins == 'E':
        wayp = (wayp[0] + val, wayp[1])
    elif ins == 'W':
        wayp = (wayp[0] - val, wayp[1])
    else:
        if ins == 'R':
            val = -val % 360
        x = wayp[0]
        y = wayp[1]
        print(wayp)
        wayp = (x * cos[val] - y * sin[val], x * sin[val] + y * cos[val])
        print(wayp)


print(abs(pos[0]) + abs(pos[1]))
