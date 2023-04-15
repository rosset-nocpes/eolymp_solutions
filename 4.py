a = list(map(float, input().split()))
def checkin(x):
    if abs(x) < 10**9:
        x = bool(x)
        return True
    else:
        return False

if all(map(checkin, a)) == True:
    if a[0] == a[3] and a[1] == a[4] and a[2] == a[5]: # infinity
        print(-1)
    elif a[2]+a[5] == abs(a[0]-a[3]): # 1
        print(1)
    elif a[2]+a[5] > abs(a[0]-a[3]): # 2
        print(2)
    elif a[2]+a[5] < abs(a[0]-a[3]): # 0
        print(0)
else:
    print("nope")
