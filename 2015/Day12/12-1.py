import json 

def countChild(jsonElement):
    count = 0

    if isinstance(jsonElement, dict):
        for child in jsonElement.values():
            count += countChild(child)

    elif isinstance(jsonElement, list):
        for child in jsonElement:
            count += countChild(child)
    elif isinstance(jsonElement, int):
        count += int(jsonElement)
    
    return count
    


document = json.loads(open("input.txt", "r").read())
print(countChild(document))
