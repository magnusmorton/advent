import sys

numbers = [int(num) for num in sys.stdin.readline().split(',')]
history = {}

for i, num in enumerate(numbers):
    history[num] = (i+1,)


turn = len(numbers)

while turn != 30000000:
    turn += 1
    last = history[num]
    if len(last) > 1:
        num = last[1] - last[0]
    else:
        num = 0
    if num in history:
        history[num] = (history[num][-1], turn)
    else:
        history[num] = (turn,)

print(num)
