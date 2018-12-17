import sys

if __name__ == '__main__':
    result = 0
    for num in sys.stdin:
        result += int(num)
    print(result)

