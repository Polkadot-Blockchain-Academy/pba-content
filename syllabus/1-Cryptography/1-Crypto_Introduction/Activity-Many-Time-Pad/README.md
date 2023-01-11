# Many Time Pad

Instructor's guide.

Review the [Many_Time_Pad_Activity.md](./Many_Time_Pad_Activity.md) file for instructions for students.
Review [PRIVATE-generate_ciphertexts-many-time-pad.py](./PRIVATE-generate_ciphertexts-many-time-pad.py) to view the keys and plaintext messages.
Provide 2 hours to crack the Many Time Pad, less if giving more hints up front to a minimum of ~30 minutes knowing them all at the start.

## Helpful Hints

As promised in the student instructions, make these suggestion _in order_.
Suggested pacing of very roughly 15 minute intervals.
Some groups get this dine _fast_ others never get there, even with all the hints listed.

### 1

We have determined the encryption method: a _python_ program!

(Share [HINT-encryption-program.py](./HINT-encryption-program.py) with students.)
(Run with `python3 HINT-encryption-program.py "<key>" "<plaintext>"`)

### 2

Recall the properties of a binary XOR:

- `a ^ b == b ^ a`
- `x ^ x == 0`
- `x ^ 0 == x`

What might this information do to help us solve our problem?

- `plaintext_1 ^ key == ciphertext_1`
- `plaintext_2 ^ key == ciphertext_2`
- `plaintext_3 ^ key == ciphertext_3`
- etc.

### 3

XOR the ciphertexts together, what is the _simplified, using XOR properties we know,_ result?

Consider what happens when a space is XORed with a character in [a-zA-Z].
https://www.utf8-chartable.de/

### 4

(NOTE: This is basically a full solution to find the key, and should unblock almost everyone)

Consider Crib dragging: https://samwho.dev/blog/toying-with-cryptography-crib-dragging/ .

### 5

Notice any patterns in the last message?
They may help uncover some hints... but are also special and may be harder to decrypt.

For the last message: in Python, strings are a lot looser with encoding changing behind the scenes to fit the present demands...
Rust is _explicit_ in all string operation to avoid any ambiguity.

Review https://doc.rust-lang.org/book/ch08-02-strings.html#internal-representation .

What will our known python encryption algorithm do when dealing with things like unicode... say, an emoji? 🤔

## Potential Solution

For instructor use only, or if no student group has found the key: https://joshorndorff.github.io/DanBonehsCryptoExercises/crib-dragging/ is a tool used to perform crib dragging interactively, can be used to demonstrate how to solve this for the class.

## How to Configure new messages

- Generate cyphertext for some text: [PRIVATE-generate_ciphertexts-many-time-pad.py](./PRIVATE-generate_ciphertexts-many-time-pad.py)
- Update the instructions to use the correct cyphertext: [Many_Time_Pad_Activity.md](./Many_Time_Pad_Activity.md)
- Update the HINT program as needed: [HINT-encryption-program.py](./HINT-encryption-program.py)
