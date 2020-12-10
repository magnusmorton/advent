import sys;

nums = [int(line) for line in sys.stdin]
nums.sort()
configs = {0: 1}
prev = [0,]

for num in nums:
    prev = [pre for pre in prev if (num - pre) < 4]
    ways = 0
    for pre in prev:
        ways += configs[pre]
    prev.append(num)
    configs[num] = ways

print(configs[num])
