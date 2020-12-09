import re
import sys

table = {}
counts = {}


def search(color):
    if not table[color]:
        print(color)
        counts[color] = 1
        return 1
    # if color in counts:
    #     return counts[color]
    count = 1
    for child in table[color]:
        print(f"{child[0]} {child[1]}")
        sub = search(child[1])
        count += sub * child[0]
    counts[color] = count
    return count

prog = re.compile(r"(\d+) (.*) bag")
for line in sys.stdin:
    splits = line.split(" bags contain")
    decl = splits[0]
    cont = splits[1].rstrip(".\n")
    if "no other" in cont:
        table[decl] = []
        continue

    colors = []
    for colo in cont.split(","):
        m = prog.search(colo)
        count = m.group(1)
        color = m.group(2)
        colors.append((int(count), color))
    table[decl] = colors

# search

print(table)
print(search("shiny gold")-1)
