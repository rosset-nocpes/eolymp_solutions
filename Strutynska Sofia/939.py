a = int(input())
if a/10 < 1 or a/100 >= 1:
    print("вхідні дані не підходять")
else:
    print((a//10 + a%10)**2)