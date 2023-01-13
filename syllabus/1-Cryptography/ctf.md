CAPTURE THE FLAG

Write each challenge on a board so they don\'92t look the next one up beforehand.

Submissions: separate txt files with names "flag_X" for each flag result

Grading: adding automation soon\

Step by step:

	CANCELLED 
	Give them a list of libraries to use\
		https://docs.rs/blake2/latest/blake2/
		https://docs.rs/ed25519/latest/ed25519/
		https://docs.rs/py-sr25519-bindings/latest/sr25519/
	CANCELLED

	Wait for the file commits

	Look at the first submission live to track the fastest who will get a beer at the end

	Grading: run flag test. The individual result for the student should be related to the mnemonic they provide at flag_0

	For sync submissions as for answers only, for async look at their code

FLAGS


FLAG_0: Mnemonic
Generate a 24 words mnemonic

Submit: 24 words mnemonic
Test: check if there are 24 words


FLAG_1: Hash
Use the fastest cryptographic hash algorithm to hash the following messages:
"Welcome to PBA 02 Buenos Aires"
[your mnemonic here]

Submit: standard hash
		hash of the mnemonic
Test: blake2("Welcome to PBA 02 Buenos Aires") == first line of FLAG_1 file
	  blake2(FLAG_0) == second like of FLAG_1 file


FLAG_2: Symmetric Encryption
Generate a Ed25519 Private and Public key pair from your hash

Submit: pub key from mnemonic hash

We will gather the list of public key and randomly assign you a public key for you to send a message to
Write a message, encrypt it with the public key assigned to you

Submit: encrypted message 

We will publish a list of all encrypted messages

Brute force the list to find the message addressed to your pubkey and decrypt it

Submit: plaint text message sent to you

Test: encrypted(plain_text_message, pub_key) == encrypted_message


FLAG_3
Signatures\cf5 \cb6 \expnd0\expndtw0\kerning0
\outl0\strokewidth0 \strokec5 \
\cf2 \cb3 \kerning1\expnd0\expndtw0 \outl0\strokewidth0 Write a message \'97 tell them it should be something they don\'92t think everyone would agree\
Hash it\
Convince your colleagues to sign your message without telling them what it is\
If someone published the plaintext for your message you loose\
Whoever has more signatures win\
	Submit: list of signatures\
	Test: check if signatures can be verified by pub keys provided in previous exercise\
\
Key derivation\
Hard derivation of your pub key\
Soft derivation of your pub key\
	Submit: first hard from pub key\
		     first soft from pub key\
	Test: check if derivation matches the pub key provided\
\
\
Erasure coding:\
Item 5 https://docplayer.net/7549450-Exercise-2-checksums-raid-and-erasure-coding.html\
\
ZK Proofs: \
	Submit: schnorr sig of text\
	Test: verify sig\
\
\
\
}