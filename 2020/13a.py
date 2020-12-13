import sys
import math

lines = [line for line in sys.stdin]

ts = int(lines[0])

buses = lines[1].split(',')
buses = [int(bus) for bus in buses if bus != 'x']
found = None
mint = 0
for bus in buses:
    fac = math.ceil(ts/bus)
    time = fac * bus
    if found:
        if time < mint:
            mint = time
            found = bus
    else:
        mint = time
        found = bus

print((mint - ts) * found)
