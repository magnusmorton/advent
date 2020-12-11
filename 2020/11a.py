import sys
import copy

grid = [list(line.rstrip()) for line in sys.stdin]

rows = len(grid)
cols = len(grid[0])
changed = True
occupied = 0
while changed:
    changed = False
    newgrid = copy.deepcopy(grid)
    for i in range(rows):
        for j in range(cols):
            adj = 0
            inds = [(i-1, j-1), (i-1, j), (i-1, j+1),
                    (i, j-1), (i, j+1),
                    (i+1, j-1), (i+1, j), (i+1, j+1)]
            inds = [(x, y) for x, y in inds
                    if x >= 0 and x < rows and y >= 0 and y < cols]
            for r, c in inds:
                if grid[r][c] == '#':
                    adj += 1
            if grid[i][j] == 'L' and adj == 0:
                newgrid[i][j] = '#'
                changed = True
                occupied += 1
            elif grid[i][j] == '#' and adj >= 4:
                newgrid[i][j] = 'L'
                changed = True
                occupied -= 1
    grid = newgrid

print(occupied)
