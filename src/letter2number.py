letters = input().split()
if len(letters[0]) > 1:
    letters = list(letters[0])
for letter in letters:
    if letter.isalpha():
        number = ord(letter) - 0x40
        print(str(number) + ", ", end='')
print("")