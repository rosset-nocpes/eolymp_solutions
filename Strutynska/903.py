a=int(input())
if a/1000>=1 or a/100<=1:
    print("вхідні дані не підходять")
else:
    if a//100 > a%10:
        print(a//100)
    elif a//100 < a%10:
        print(a%10)
    else:
        print("=")