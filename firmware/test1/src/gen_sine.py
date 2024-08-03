import math

n = 512
offset = 2048
headroom = 256  # e.g. for adding noise
amplitude = 2048 - headroom
x = []
for i in range(n):
    if n/2 <= i <= 3*n/4 and True:
        y = offset - amplitude
    elif n/4 <= i <= 3*n/4 and True:
        y = offset + amplitude - (i-n/4)*2*amplitude/(n/2)
    else:
        y = offset + amplitude*math.sin(i*2*math.pi/n)
    x.append(min(4095-headroom, max(headroom, int(y))))
#print(repr(x))

per_line = 16
print("[")
for i in range(0, len(x), per_line):
    print("    " + ", ".join(map(str, x[i:i+per_line])) + ",")
print("]")
