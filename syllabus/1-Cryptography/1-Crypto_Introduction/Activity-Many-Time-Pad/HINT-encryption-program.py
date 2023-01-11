import sys

def strxor(a, b):     # xor two strings of different lengths
    if len(a) > len(b):
       return "".join([chr(ord(x) ^ ord(y)) for (x, y) in zip(a[:len(b)], b)])
    else:
       return "".join([chr(ord(x) ^ ord(y)) for (x, y) in zip(a, b[:len(a)])])

def encrypt(key, msg):
    ciphertext = strxor(key, msg)
    return ciphertext

def string_to_hex(s):
    return "".join("{:02x}".format(ord(c)) for c in s)

######### Main Program

key = sys.argv[1]
plaintext = sys.argv[2]

ciphertext = encrypt(key, plaintext)

print(string_to_hex(ciphertext))