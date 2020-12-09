import sys
from collections import deque


nums = [int(line) for line in sys.stdin]
target = 20874512
window = deque()
running = 0
for num in nums:
    running += num
    window.append(num)
    while running > target:
        running -= window.popleft()

    if len(window) > 1 and running == target:
        print(min(window) + max(window))
