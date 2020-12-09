import re
import sys

table = {}
counts = {}


def search(color):
    if color == "shiny gold":
        return
    for child in table[color]:
        search(child)
        if child == "shiny gold" or child in counts:
            counts[color] = 1
            continue


prog = re.compile(r"\d+ (.*) bag")
for line in sys.stdin:
    splits = line.split(" bags contain")
    decl = splits[0]
    cont = splits[1].rstrip(".\n")
    if "no other" in cont:
        table[decl] = []
        continue

    colors = []
    for colo in cont.split(","):
        print(colo)
        m = prog.search(colo)
        color = m.group(1)
        colors.append(color)
    table[decl] = colors

# search
for decl in table:
    search(decl)

print(len(counts))
