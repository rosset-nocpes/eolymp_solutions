x1, y1, r1, x2, y2, r2 = map(float, input().split())

с = abs(((x2 - x1)**2 + (y2 - y1)**2)**0.5)

if с > r1 + r2 or с < abs(r2 - r1):
    print(0)
elif с == 0 and r2 == r1:
    print(-1)
elif с == r2 + r1 or с == abs(r2 - r1):
    print(1)
else:
    print(2)
