import collections
import re
import sys


Rectangle = collections.namedtuple('Rectangle', ['id', 'x', 'y', 'width', 'height'])


class Rectangle(Rectangle):

    @property
    def coords(self):
        for x in range(self.x, self.x + self.width):
            for y in range(self.y, self.y + self.height):
                yield (x, y)


RECT_NOTATION = re.compile(r'#([\d]+) @ (\d+),(\d+): (\d+)x(\d+)')


def parse_rect_notation(line):
    # '#99 @ 433,123: 26x53'
    match = RECT_NOTATION.match(line)
    assert match, line
    id_, x, y, width, height = match.groups()
    return Rectangle(int(id_), int(x), int(y), int(width), int(height))



def read_rectangles(lines):
    return [parse_rect_notation(l) for l in lines]


if __name__ == '__main__':
    lines = [line.strip() for line in sys.stdin.readlines()]

    rectangles = read_rectangles(lines)
    grid = collections.defaultdict(list)

    for rect in rectangles:
        for coord in rect.coords:
            grid[coord].append(rect)

    possible_solutions = set(r.id for r in rectangles)
    for _, rects in grid.items():
        if len(rects) > 1:
            for rect in rects:
                if rect.id in possible_solutions:
                    possible_solutions.remove(rect.id)
    print(possible_solutions)
