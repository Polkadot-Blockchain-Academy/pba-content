Capture the flag

1. Hash the message "Welcome to PBA 02 Buenos Aires"
    Commit the hash

2. Hash the message "Welcome to PBA 02 Buenos Aires, [INSERT YOUR ELEMENT HANDLE]"
    Commit the hash

3. Generate a private key, send it to one of your colleagues
    XOR Encrypt "Welcome to PBA 02 Buenos Aires, [INSERT YOUR ELEMENT HANDLE]"
    Both send me their encrypted messages
    We will publish a list with the message pairs, you should pick any pair that is not yours and try to engineer whose pair it is
    Yes you can just ask your colleagues but those who submit a guess will either have to present their technique to the group or submit it written for evaluation. Submissions of answers without engineering process will not be accepted

4. Generate a private and public key pair
    Commit your public key
    We will gather the list of public key and randomly assign you a public key for you to send a message to
    Write a message, encrypt it with the public key assigned to you
    Commit your encrypted message
    Send us the hash of your plain text message
    Brute force the list to find the message addressed to your pubkey
    Commit the hash of the received message
    We will publish the list of hashes and the only answers accepted will be commited hashes that are on the list

5. With your previous key pair
    Key signing party with your colleagues
    Commit all signatures you got
    Your final points will be calculated based on the amout of TRUE signatures you have

6. Hard derive a grandchild to your key TODO

7. Soft derive 5 addresses from your key
    Commit all addresses 

8. Generate a 24 words mnemonic
    Try to run it in this list of key derivation algorithms TODO
    Commit the hash of your answers to if it breaks (1) or not (0) IN ORDER

9. 
