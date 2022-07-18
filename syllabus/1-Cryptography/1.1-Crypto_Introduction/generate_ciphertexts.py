import sys
import random

# Inspired by Dan Boneh's Coursera: https://www.coursera.org/learn/crypto/quiz/KZ9js/week-1-programming-assignment-optional/attempt

ABSTRACT = "A purely peer-to-peer version of electronic cash would allow online payments to be sent directly from one party to another without going through a financial institution. Digital signatures provide part of the solution, but the main benefits are lost if a trusted third party is still required to prevent double-spending. We propose a solution to the double-spending problem using a peer-to-peer network. The network timestamps transactions by hashing them into an ongoing chain of hash-based proof-of-work, forming a record that cannot be changed without redoing the proof-of-work. The longest chain not only serves as proof of the sequence of events witnessed, but proof that it came from the largest pool of CPU power. As long as a majority of CPU power is controlled by nodes that are not cooperating to attack the network, they’ll generate the longest chain and outpace attackers. The network itself requires minimal structure. Messages are broadcast on a best effort basis, and nodes can leave and rejoin the network at will, accepting the longest proof-of-work chain as proof of what happened while they were gone."

MSGS = [
    "The Times 03/Jan/2009 Chancellor on brink of second bailout for banks.",
    "Governments are good at cutting off the heads of a centrally controlled networks like Napster, but pure P2P networks like Gnutella and Tor seem to be holding their own.",
    "Bitcoin is great as a form of digital money, but its scripting language is too weak for any kind of serious advanced applications to be built on top.",
    "n order to have a decentralised database, you need to have security. In order to have security, you need to have incentives.",
    "As society becomes more and more complex, cheating will in many ways become progressively easier and easier to do and harder to police or even understand.",
    "I began to realise new possibilities opening up between the fields of ICT and game theory, and the inevitable social change to which this would lead.",
    "Cryptocurrencies allowed non-custodial exchange, without users having to sign up or create accounts.",
    "Not your keys, Not your coins.",
]

# This is the output from the above ciphertexts:
# 15481555260c011c535055565d671501024255554b00350d131d0a0a024c0014000a024501061b0105490c464312160b4f190b550e05490d03191b5746001c4c0b0f0b4b034f
# 064f0610000b011c4e041645135f114f4a1f0a015241024511061d1b074e08464f030a45171c174f060c024410411c0e00164f16090a54130d00030e000c01021d1c0a4c1c041d4d0b0b00044f06045342094918004e3a41141a060011584c1b5512521f18520a4e351220411c110d571b1d4b124e031d03005267191c000d0319150006010d4e334f0648010a100a48540e00040c4e09010f0d080247491a1b110006551b1e0140
# 034904161d0c0259490345020048151b0d1116451300100a001e490008000b0f470c18040f541f00000c1a0c4303061c001e1b064c174313051c1b1e4e084e000800025511061c4d0c1d54074f1b4f5707044b530301060005070b45081d021d0009144f1e451d070a55034113100f411a0c45054e0e0418091b43161d1d0701065454084f0b0b474201011e1b55080600154f1647
# 2f001f0716001e59541f450d135b114f4c50010011451811001205061d450b46440418040115010a42491a4f16411d0d45134f01034448001a094f04450c1b1e001a1c0e5028174d0a1c101652541b4f420d4105004e0745071c000c170d40595909074f03450a0a45541f411a150f4554064e020b01000113175359
# 005350061d06051c54094507174e1b02480345081d521345131d0d4f034f1d03000603081318171742490048060007014e104f0205084c4105024f1a4101174c1e0f1c5350031c0e0a03115350060047100053000c18114c1d491704101d090b00071c0b4d450e1d0c450241061b59441b4f410f0a4f1c09171645054900074f051b4c0e0c0c4e0852540d040a1b471d4e0545141a1a00000747
# 0800121015040259541f4517174c18065e15450b175756151d001a060c49030f540c0916431b020a00000d474314034842121b0209014e4118040a5746060b000d1d454f1641302e314e151d445408410f0000070d0b1b521d4552040d104c0d48035206034519071141120d17540a4f1706410d4e0c1c090b1545571d1b48181d1d430f4f1d060e53541f1d1a1903484c04410247
# 02520905060a0f0c5202000b1144111c0d1109091d571301521d060143431a15540a080c0218520a160a0b410d06164400000601040b55154c191c12521c4e0408180c4e17410d02451d1d144e541a50420a5253061c1141100c52040017030c4e120141
# 0f4f04550b0a190b001b001c010154214204451c1d550445111c00011d0e




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
key = ABSTRACT[0:longest]
# key = generate_random_key(longest)
# print("The key is {}".format(key))

ciphertexts = [encrypt(key, msg) for msg in MSGS]

for (p, c) in zip(MSGS, ciphertexts):
    p_hex = string_to_hex(p)
    c_hex = string_to_hex(c)

    # print("plaintext {} {}".format(len(p), len(p_hex)))
    # print("ciphertext {} {}".format(len(c), len(c_hex)))

    # print(p_hex)
    print(c_hex)

    # print()

