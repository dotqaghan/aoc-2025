import sys
import itertools
from shapely.geometry import Polygon, box


def parse(file):
    points = []
    with open(file, 'r') as file:
        for line in file:
            line = line.strip()
            if not line: continue
            parts = line.split(',')
            points.append((int(parts[0]), int(parts[1])))

    return points


def part2(points):
    poly = Polygon(points)
    max_area = 0
    best_rect = None

    for p1, p2 in itertools.combinations(points, 2):
        x1, y1 = p1
        x2, y2 = p2

        min_x, max_x = min(x1, x2), max(x1, x2)
        min_y, max_y = min(y1, y2), max(y1, y2)
        candidate_rect = box(min_x, min_y, max_x, max_y)

        # check bounds
        if poly.contains(candidate_rect):
            area = candidate_rect.area
            if area > max_area:
                max_area = area
                best_rect = (min_x, min_y, max_x, max_y)

    return best_rect


if __name__ == "__main__":
    filename = sys.argv[1]
    coords = parse(filename)
    (x1, y1, x2, y2) = part2(coords)

    print(f"result 2: {(x2 - x1 + 1) * (y2 - y1 + 1)}")