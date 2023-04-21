a = int(input())

first = a // 100
last = a % 10

if first > last:
    print(first)
elif first < last:
    print(last)
else: 
    print("=")
