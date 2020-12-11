import sys
import copy
import itertools

grid = [list(line.rstrip()) for line in sys.stdin]

rows = len(grid)
cols = len(grid[0])


def cast(pos, direction):
    r, c = pos
    ri, ci = direction
    stopr = rows if ri > 0 else -1
    stopc = cols if ci > 0 else -1
    while r != stopr and c != stopc:
        if pos != (r, c) and grid[r][c] != '.':
            return (r, c)
        r += ri
        c += ci


directions = [(x, y) for x, y in itertools.permutations([-1, 0, 1], 2)]

directions.append((1, 1))
directions.append((-1, -1))

nearest = [[[cast((row, col), di) for di in directions]
            for col in range(cols)] for row in range(rows)]

changed = True
occupied = 0
while changed:
    changed = False
    newgrid = copy.deepcopy(grid)
    for i in range(rows):
        for j in range(cols):
            adj = 0
            for pos in nearest[i][j]:
                if pos is not None and grid[pos[0]][pos[1]] == '#':
                    adj += 1
            seat = grid[i][j]
            if seat == 'L' and adj == 0:
                newgrid[i][j] = '#'
                changed = True
                occupied += 1
            elif seat == '#' and adj >= 5:
                newgrid[i][j] = 'L'
                changed = True
                occupied -= 1
    grid = newgrid

print(occupied)
