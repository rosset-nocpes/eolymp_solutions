a = list(map(float, input().split()))

distance = ((a[3] - a[0])**2 + (a[4] - a[1])**2)**0.5

if distance > a[2] + a[5]:
    print(0)
elif distance == 0 and a[5] == a[2]:
    print(-1)
else:
    if distance == a[5] + a[2] or distance == abs(a[5] - a[2]):
        print(1)
    elif distance < abs(a[5] - a[2]):
        print(0)
    else:
        print(2)
