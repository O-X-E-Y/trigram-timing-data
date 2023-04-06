import json

with open("./data/0.json", 'r', encoding='utf8') as f:
    data = json.load(f)

keys = set()

for trigram in data.keys():
    for key in trigram.split(","):
        keys.add(key)

print(keys)