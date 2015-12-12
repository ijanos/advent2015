import json
    
def summarize(data):
    if type(data) == dict:
        return sum([summarize(x) for x in data.values()])
    if type(data) == list:
        return sum([summarize(x) for x in data])
    if type(data) == int:
        return data
    return 0
    
data = json.load(open('input.txt'))

print("Answer:", summarize(data))