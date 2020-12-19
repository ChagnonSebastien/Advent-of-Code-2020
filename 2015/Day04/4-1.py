import hashlib


input = "ckczppom"

i = 1
hash = "12345"

while hash[0:5] != "00000":
    i += 1
    m = hashlib.md5((input + str(i)).encode('utf-8'))
    hash = m.hexdigest()

print(i)