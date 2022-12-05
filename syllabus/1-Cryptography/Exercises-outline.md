Capture the flag

HASH
1. Hash the message "Welcome to PBA 02 Buenos Aires"
    Commit the hash

2. Hash the message "Welcome to PBA 02 Buenos Aires, [INSERT YOUR ELEMENT HANDLE]"
    Commit the hash

SYMETRIC
3. Generate a private key, send it to one of your colleagues
    XOR Encrypt "Welcome to PBA 02 Buenos Aires, [INSERT YOUR ELEMENT HANDLE]"
    Both send me their encrypted messages
    We will publish a list with the message pairs, you should pick any pair that is not yours and try to engineer whose pair it is
    Yes you can just ask your colleagues but those who submit a guess will either have to present their technique to the group or submit it written for evaluation. Submissions of answers without engineering process will not be accepted

ASYMETRIC
4. Generate a private and public key pair
    Commit your public key
    We will gather the list of public key and randomly assign you a public key for you to send a message to
    Write a message, encrypt it with the public key assigned to you
    Commit your encrypted message
    Send us the hash of your plain text message
    Brute force the list to find the message addressed to your pubkey
    Commit the hash of the received message
    We will publish the list of hashes and the only answers accepted will be commited hashes that are on the list

SIGNATURES
5. With your previous key pair
    Key signing party with your colleagues
    Commit all signatures you got
    Your final points will be calculated based on the amout of TRUE signatures you have

KEY DERIVATION
6. Hard derive a grandchild to your key TODO

7. Soft derive 5 addresses from your key
    Commit all addresses 

MNEMONICS
8. Generate a 24 words mnemonic
    Try to run it in this list of key derivation algorithms TODO
    Commit the hash of your answers to if it breaks (1) or not (0) IN ORDER

CERTIFICATES
9. TODO

MULTISIG
10. Gather 5 colleagues:
    Decide for a common message
    Sign the message with threshold multisig
    Commit signature + verifying key
    Sign the message with non-threshold multisig
    Commit signatures + verifying key
    N-1 Sign the message with ring signature
    Commit signature + verifying key
    We will randomly assing you 3 groups (one for each signature type) for your group to verify
    Commit verification of each group

VRF
11. Cards Against Blockchain ?

ERASURE CODING
12. We will publish a list of information chunks, all with redundance X so that there are 15% corrupted and they can still get the message back with not too many chunks
    Ask for them to rebuild the text
    Commit text
    Either everyone get points or no-one does

ZK PROOFS
13. Something with Schnorr signatures TODO
     Maybe give them a big chunk of data that they should compress like a rollup type of scheme as a group
     Or do a random oblivious transfer type of thing with each



