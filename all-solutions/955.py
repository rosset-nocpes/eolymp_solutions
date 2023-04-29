n = int(input())
first = n//1000
second = (n//100)%10
third = (n//10)%10
fourth = n%10
print((first+second+third+fourth)**2)
