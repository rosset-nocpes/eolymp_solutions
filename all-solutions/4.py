import math

x1, y1, r1, x2, y2, r2 = map(float, input().split())
dl = math.sqrt((x2-x1)**2 + (y2-y1)**2)

if r2 > r1:
    f=r1
    r1=r2
    r2=f
    f=x2
    x1=x2
    x2=f
    f=y1
    y1=y2
    y2=f

if x1 == x2 and y1 == y2 and r1 == r2:
    print("-1")
elif r1 + r2 == dl or dl == r1 - r2:
    print("1")
elif r1 + r2 < dl or dl < r1 - r2:
    print("0")
else:
    print("2")