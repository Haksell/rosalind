n, k = map(int, open("rosalind_fib.txt").read().split())

a, b = 1, 1
for _ in range(n - 1):
    a, b = b, k * a + b
print(a)
