[print("".join("".join([[p for p in layer if p != "2"][0] for layer in zip(*[open("input", "r").read().rstrip()[i:i+(6*25)] for i in range(0, len(open("input", "r").read().rstrip()), (6*25))])])[i*25: i*25 + 25]).replace("0", " ")) for i in range(6)]
