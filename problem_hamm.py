s, t = open("rosalind_hamm.txt").read().strip().split()
print(sum(si != ti for si, ti in zip(s, t)))
