import re

def nextPassword(lastPassword):
    password = [c for c in lastPassword]
    overflow = True
    i = len(password) - 1
    while overflow:
        overflow = False

        nextChar = ord(password[i]) + 1
        if nextChar == 123:
            overflow = True
            nextChar = 97

        password[i] = chr(nextChar)
        i -= 1
    
    return ''.join(password)
        

password = "vzbxkghb"

firstCondition = '^[^iol]*$'
secondCondition = 'abc|bcd|cde|def|efg|fgh|ghi|hij|ijk|jkl|klm|lmn|mno|nop|opq|pqr|qrs|rst|stu|tuv|uvw|vwx|wxy|xyz'
thirdCondition = '(.)\\1{1}.*(.)\\2{1}'

for _ in range(2):
    password = nextPassword(password)
    while re.search(firstCondition, password) is None or re.search(secondCondition, password) is None or re.search(thirdCondition, password) is None:
        password = nextPassword(password)

print(password)