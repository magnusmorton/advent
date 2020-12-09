import sys
from collections import deque

def compute_sums(numset):
    sums = set()
    for num in numset:
        for oth in numset:
            if num != oth:
                sums.add(num + oth)
    return sums


nums = [int(line) for line in sys.stdin]

preamble = deque(nums[0:25])


for num in nums[25:]:
    sums = compute_sums(preamble)
    if num not in sums:
        print(num)
        break
    preamble.popleft()
    preamble.append(num)
