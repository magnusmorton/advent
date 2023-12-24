import re
import sys

total = 0
for line in sys.stdin:
    possible = False
    header, rounds_str = line.split(':')

    game_id = int(header.split()[1])
    rounds = rounds_str.split(';')
    max_red = 0
    max_green = 0
    max_blue = 0
    for rnd in rounds:
        for pair in rnd.split(','):
            num, colour  = pair.split()
            count = int(num)
            match colour.rstrip():
                case "red":
                    if count > max_red:
                        max_red = count
                case "blue":
                    if count > max_blue:
                        max_blue = count
                case "green":
                    if count > max_green:
                        max_green = count
    total += max_red * max_green * max_blue
print(total)
                    
