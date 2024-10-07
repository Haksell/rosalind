from collections import Counter

cnt = Counter(open("rosalind_dna.txt").read().strip())
print(cnt["A"], cnt["C"], cnt["G"], cnt["T"])
