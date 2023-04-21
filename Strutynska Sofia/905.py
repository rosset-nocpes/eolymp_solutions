a, b, c = map(int, input().split())
if a < b + c and b < a + c and c < b + a:
    if a==b==c:
        print("1")
    elif (a==b and a!=c) or (a==c and a!=b) or (b==c and b!=a):
        print("2")
    else:
        print("3")
else:
    print("трикутника не існує")