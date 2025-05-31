#!/usr/bin/python3

from collections import namedtuple
from pprint import pprint


Line = namedtuple('Line', ['name', 'hexcode', 'bincode', 'arg', 'addr'])


def parse_line(l):
    name, hexcode, bincode, arg, addr = l.strip().split()
    return Line(
        name,
        int(hexcode, 16),
        bincode,
        arg,
        addr
    )


with open('ins-bin.txt') as f:
    lines = [parse_line(l) for l in f.readlines() if len(l) > 1]

# lines = lines[::-1][:10]
lines.sort(key=lambda e: e[1])

for l in lines:
    print(f'{l.hexcode:x} {l.name}')
