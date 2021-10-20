import sys

numbers = [int(num) for num in sys.stdin.readline().split(',')]
history = {}

for i, num in enumerate(numbers):
    history[num] = i+1


turn = len(numbers)

prev = num
while turn != 30000000:
    turn += 1
    last = history[num]
    num = 0
    if num in history:
        history[num] = (history[num][-1], turn)
    else:
        history[num] = (turn,)

print(num)
