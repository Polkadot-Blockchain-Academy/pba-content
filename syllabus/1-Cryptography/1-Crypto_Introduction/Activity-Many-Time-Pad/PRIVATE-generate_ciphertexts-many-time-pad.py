import sys
import random

# Inspired by Dan Boneh's Coursera: https://www.coursera.org/learn/crypto/quiz/KZ9js/week-1-programming-assignment-optional/attempt

KEY_SOURCE = "Bitcoin: A purely peer-to-peer version of electronic cash would allow online payments to be sent directly from one party to another without going through a financial institution. Digital signatures provide part of the solution, but the main benefits are lost if a trusted third party is still required to prevent double-spending. We propose a solution to the double-spending problem using a peer-to-peer network. The network timestamps transactions by hashing them into an ongoing chain of hash-based proof-of-work, forming a record that cannot be changed without redoing the proof-of-work. The longest chain not only serves as proof of the sequence of events witnessed, but proof that it came from the largest pool of CPU power. As long as a majority of CPU power is controlled by nodes that are not cooperating to attack the network, they’ll generate the longest chain and outpace attackers. The network itself requires minimal structure. Messages are broadcast on a best effort basis, and nodes can leave and rejoin the network at will, accepting the longest proof-of-work chain as proof of what happened while they were gone."

MSGS = [
    "The Times 03/Jan/2009 Chancellor on brink of second bailout for banks.",
    "Governments are good at cutting off the heads of a centrally controlled networks like Napster, but pure P2P networks like Gnutella and Tor seem to be holding their own.",
    "Bitcoin is great as a form of digital money, but its scripting language is too weak for any kind of serious advanced applications to be built on top.",
    "In order to have a decentralized database, you need to have security. In order to have security, you need to have incentives.",
    "As society becomes more and more complex, cheating will in many ways become progressively easier and easier to do and harder to police or even understand. ",
    "I began to realize new possibilities opening up between the fields of ICT and game theory, and the inevitable social change to which this would lead.",
    "Cryptocurrencies allowed non-custodial exchange, without users having to sign up or create accounts.",
    "Not your keys, Not your coins.",
    "Bitcoin: A purely peer-to-peer version of electronic cash would allow online payments to be sent directly from one party to another without going through a financial institution.",
    "🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉 Congratulations on championing the first of many assignments here at the Polkadot Blockchain Academy! We are so glad to have you here! 🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉",
]

def strxor(a, b):     # xor two strings of different lengths
    if len(a) > len(b):
       return "".join([chr(ord(x) ^ ord(y)) for (x, y) in zip(a[:len(b)], b)])
    else:
       return "".join([chr(ord(x) ^ ord(y)) for (x, y) in zip(a, b[:len(a)])])

def encrypt(key, msg):
    ciphertext = strxor(key, msg)
    return ciphertext

def generate_random_key(length):  # Returns a string of the given length composed of random ascii characters
    s = ""

    for i in range(length):
        s += chr(random.randint(0, 128))
    
    return s

def string_to_hex(s):
    return "".join("{:02x}".format(ord(c)) for c in s)


######### Main Program

# Find the length of the longest input
longest = 0
for msg in MSGS:
    if len(msg) > longest:
        longest = len(msg)

# Generate a key. You can generate a random one or use this specific one.
key = KEY_SOURCE[0:longest]
# key = generate_random_key(longest)
print("\nThe KEY is:\n{}\n\n".format(key))

ciphertexts = [encrypt(key, msg) for msg in MSGS]

for (p, c) in zip(MSGS, ciphertexts):
    p_hex = string_to_hex(p)
    c_hex = string_to_hex(c)

    # print("plaintext {} {}".format(len(p), len(p_hex)))
    # print("----------\nplaintext: \"{}\"".format(p))

    # print(p_hex)
    # print("ciphertext (hex):\n{}\n".format(c_hex))
    print(c_hex)

    # print()

