dna = open("rosalind_revc.txt").read().strip()
replaced = dna.translate(str.maketrans("ACGT", "TGCA"))
print(replaced[::-1])
