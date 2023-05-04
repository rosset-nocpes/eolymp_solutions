m, n, k = map(int, input().split())
print(f"{m // n}.", end='')
d = m % n
for i in range(k):
  d *= 10
  ratio2 = d // n
  print(ratio2, end='')
  d %= n