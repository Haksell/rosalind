best_p = 0.0
best_name = ""
for dna_string in open("rosalind_gc.txt").read().strip().split(">")[1:]:
    name, *lines = dna_string.split("\n")
    content = "".join(lines).strip()
    gc = sum(c == "C" or c == "G" for c in content)
    p = gc / len(content)
    if p > best_p:
        best_p = p
        best_name = name
print(best_name, 100 * best_p, sep="\n")
