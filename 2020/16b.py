import sys
import re

lines = sys.stdin.read()

sections = lines.split('\n\n')

rules = sections[0].split('\n')

prog = re.compile(r"(.*): (.*) or (.*)")
valid = set()
ruleset = {}
for rule in rules:
   m = prog.match(rule)
   rm1 = m.group(2).split('-')
   rm2 = m.group(3).split('-')
   r1 = range(int(rm1[0]), int(rm1[1]) + 1)
   r2 = range(int(rm2[0]), int(rm2[1]) + 1)
   s1 = {i for i in r1} 
   s2 = {i for i in r2} 
   valid = valid | s1 | s2
   if 'departure' in m.group(1):
       ruleset[m.group(1)] = s1| s2

mine = sections[1].split('\n')[1].split(',')

names = [set(ruleset.keys())] * len(mine)

for near in sections[2].split('\n')[1:-1]:
    fields = [int(f) for f in near.split(',')]
    if [f for f in fields if f not in valid]:
        continue
    for i,f in enumerate(fields):
        names[i] = {name for name in names[i] if f in ruleset[name]}

import itertools

maxlen = None
converged = False
while not converged:
    converged = True
    for i,name in enumerate(names):
        if len(name) == 1:
            only = list(name)[0]
            for name2 in names:
                if name2 is not name and only in name2:
                    name2.remove(only)
                    converged = False
prod = 1 
for i, val in enumerate(mine):
    num = int(val)
    if names[i]:
        prod *= num

print(prod)
