import sys
import random
from colorsys import hsv_to_rgb


def fmtcol(c):
    return '{:X}{:X}{:X}'.format(*(int(x * 16) for x in c))


def gencol():
    x = random.random()
    return hsv_to_rgb(x, 0.8, 0.8), hsv_to_rgb(x + 0.5, 0.8, 0.8)

for line in sys.stdin.readlines():
    a, b = gencol()
    comment = ""
    if len(sys.argv) >= 2:
        comment = sys.argv[1]

    print('{code}\n!fg:{fg} !bg:{bg} {comment}'.format(
        code=line.strip(),
        fg=fmtcol(a),
        bg=fmtcol(b),
        comment=comment
    ))
