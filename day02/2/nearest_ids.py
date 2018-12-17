import sys


def diff(id1, id2):
    assert len(id1) == len(id2)
    different = 0
    differences = []
    for idx, char in enumerate(id1):
        if char != id2[idx]:
            different += 1
            differences.append((idx, char, id2[idx]))
    return different, differences



if __name__ == '__main__':
    ids = sys.stdin.readlines()
    ids = [id_.strip() for id_ in ids]

    for id1 in ids:
        for id2 in ids:
            num_different, diffs = diff(id1, id2)
            if num_different == 1:
                print(id1)
                print(id2)
                print('common letters:', id1[0:diffs[0][0]] + id1[diffs[0][0] + 1 : ])
                sys.exit(0)

    print('not found')
    sys.exit(1)
