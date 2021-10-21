import sys
import re

lines = sys.stdin.read()

sections = lines.split('\n\n')

rules = sections[0].split('\n')

prog = re.compile(r"(.*): (.*) or (.*)")
ruleset = set()
for rule in rules:
   m = prog.match(rule)
   r1 = m.group(2).split('-')
   r2 = m.group(3).split('-')
   s1 = {i for i in range(int(r1[0]), int(r1[1]) + 1)} 
   s2 = {i for i in range(int(r2[0]), int(r2[1]) + 1)} 
   ruleset = ruleset | s1 | s2

err = 0
for near in sections[2].split('\n')[1:]:
    if not near:
        continue
    fields = [int(f) for f in near.split(',')]
    err += sum(f for f in fields if f not in ruleset)

print(err)

