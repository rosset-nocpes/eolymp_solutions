n = int(input())
if n != 0:
    temp = 0
    while n > 0:
        temp += 1
        n //= 10


    print(temp)
else:
    print(1)