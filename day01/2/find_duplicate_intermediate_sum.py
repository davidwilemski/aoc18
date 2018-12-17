import itertools
import sys

if __name__ == '__main__':
    result_sum = 0
    previous_results = {result_sum}
    nums = sys.stdin.readlines()
    nums = [int(n) for n in nums]
    for num in itertools.cycle(nums):
        result_sum += int(num)
        if result_sum in previous_results:
            break
        previous_results.add(result_sum)
    print(result_sum)

