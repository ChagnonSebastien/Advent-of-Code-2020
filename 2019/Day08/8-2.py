width = 25
height = 6

data = open("input.txt", "r").read().rstrip()
layers = [data[i*width*height:(i+1)*width*height] for i in range(int(len(data)/(width * height)))]

image = layers[0]

for layer in layers:
    for i in range(width*height):
        image = image[:i] + (layer[i] if image[i] == '2' else image[i]) + image[i + 1:]

for i in range(width*height):
    image = image[:i] + ('o' if image[i] == '1' else ' ') + image[i + 1:]

for i in range(height):
    print(image[i*width:(i+1)*width])
