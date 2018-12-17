import collections
import sys


def score_word(id_):
    counter = collections.Counter(id_)
    top_two = counter.most_common(2)
    result = 0
    first = top_two[0][1]
    second = top_two[1][1]

    has_three = False
    has_two = False

    if first >= 3:
        has_three = True

        if second >= 2:
            has_two = True

    elif first >= 2 or second >= 2:
        has_two = True

    return (int(has_three), int(has_two))


if __name__ == '__main__':
    scores = []
    for id_ in sys.stdin:
        scores.append(score_word(id_))

    num_threes = sum(x[0] for x in scores)
    num_twos = sum(x[1] for x in scores)
    print(num_threes * num_twos)

