CAPTURE THE FLAG

Write each challenge on a board so they don\'92t look the next one up beforehand.

Submissions: separate txt files with names "flag_X" for each flag result

Grading: adding automation soon\

Step by step:

	CANCELLED 
	Give them a list of libraries to use\
		https://docs.rs/blake2/latest/blake2/
		https://docs.rs/bip32/latest/bip32/
		secp256k1
	CANCELLED

	Wait for the file commits

	Look at the first submission live to track the fastest who will get a beer at the end

	Grading: run flag test. The individual result for the student should be related to the mnemonic they provide at flag_0

	For sync submissions as for answers only, for async look at their code

FLAGS


FLAG_0: Mnemonic
Generate a 24 words BIP39 mnemonic

Submit: 24 words mnemonic
Test: check if there are 24 words


FLAG_1: Hash
Use the fastest cryptographic hash algorithm in size 256bits to hash the following messages:
[your mnemonic here]

Submit: standard hash
		hash of the mnemonic
Test: blake2("Welcome to PBA 02 Buenos Aires") == first line of FLAG_1 file
	  blake2(FLAG_0) == second like of FLAG_1 file


FLAG_2: Symmetric Encyption
Generate a BIP32 key pair from FLAG_1 as Private key from your hash

Submit: pub key from mnemonic hash

We will gather the list of public key and randomly assign you a public key for you to send a message to
Write a message, encrypt it with the public key assigned to you

Submit: encrypted message 

We will publish a list of all encrypted messages

Brute force the list to find the message addressed to your pubkey and decrypt it

Submit: plaint text message sent to you

Test: 
	pub_key(FLAG_2) == pub_key
	encrypted(plain_text_message, pub_key) == encrypted_message


FLAG_3: Signatures

Write a message
Blake2 the message

Submit: message hash

Convince your colleagues to sign your message without telling them what it is.
If someone published the plaintext for your message you loose.
15min to gather signatures
Whoever has more signatures win

Submit: list of signatures
	
Test: count = 0, for signature: if verify(signature, any(pub_key)) count++, else return YOU LOST
	highest count wins
	  

FLAG_4: Key derivation

https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki#user-content-Specification_Key_derivation
https://trezor.io/learn/a/what-is-bip32
https://www.blockplate.com/blogs/blockplate/list-of-bip39-wallets-mnemonic-seed#:~:text=BIP39%20is%20a%20standard%20that,%2C%20wallet%20back%20up%2C%20etc.

From your FLAG_0:

Submit:
	BIP32 root key
	BIP32 extended private key
	BIP32 extended public key
	BIP32 m/0'/0'/0 address

Test: https://iancoleman.io/bip39/#english with their mnemonic

FLAG_5: Merkle tree

