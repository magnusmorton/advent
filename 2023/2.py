import re
import sys

RED = 12
GREEN = 13
BLUE = 14

total = 0
for line in sys.stdin:
    possible = False
    header, rounds_str = line.split(':')

    game_id = int(header.split()[1])
    rounds = rounds_str.split(';')

    for rnd in rounds:
        for pair in rnd.split(','):
            num, colour  = pair.split()
            count = int(num)
            match colour.rstrip():
                case "red":
                    if count > RED:
                        break
                case "blue":
                    if count > BLUE:
                        break
                case "green":
                    if count > GREEN:
                        break
        else:
            continue
        break
    else:
        possible = True
    if possible:
        total += game_id

print(total)
                    
