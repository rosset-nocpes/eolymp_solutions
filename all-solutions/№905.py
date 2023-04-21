a, b, c = map(int, input().split())
if a==b and b==c and a==c:
    print(1)
elif a==b or a==c or c==b:
    print(2)
else:
    print(3)
