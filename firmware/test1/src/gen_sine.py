import math

n = 512
offset = 2048
headroom = 256  # e.g. for adding noise
amplitude = 2048 - headroom
x = []
for i in range(n):
    y = int(offset + amplitude*math.sin(i*2*math.pi/n))
    x.append(min(4095-headroom, max(headroom, y)))
#print(repr(x))

per_line = 16
print("[")
for i in range(0, len(x), per_line):
    print("    " + ", ".join(map(str, x[i:i+per_line])) + ",")
print("]")
