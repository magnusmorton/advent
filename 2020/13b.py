import sys

lines = [line for line in sys.stdin]
buses = lines[1].split(',')


period = 1
ts = 0
for bus in buses:
    if bus != 'x':
        bus = int(bus)
        while (mod := ts % bus) != 0:
            ts += period
        period *= bus
    ts += 1

print(ts - len(buses))
