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
    print('!fg:{} !bg:{} {} #bbcurated'.format(
        fmtcol(a),
        fmtcol(b),
        line.strip(),
    ))
