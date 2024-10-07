k, m, n = map(int, open("rosalind_iprb.txt").read().split())
s = k + m + n
total = s * (s - 1) // 2
dominant = k * (n + m) + k * (k - 1) // 2 + m * (m - 1) * 3 / 8 + m * n / 2
print(dominant / total)
