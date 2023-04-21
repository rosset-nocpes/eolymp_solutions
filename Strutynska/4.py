x1, y1, r1, x2, y2, r2 = map(float, input().split())
c = abs(((y1-y2)**2 + (x2 - x1)**2)**0.5)
if r1==r2 and x1==x2 and y1==y2:
    print("-1")
elif c > abs(r1 + r2) or c < abs(r1 - r2):
    print("0")
elif c == abs(r1 + r2) or c == abs(r1 - r2):
    print("1")
else:
    print("2")