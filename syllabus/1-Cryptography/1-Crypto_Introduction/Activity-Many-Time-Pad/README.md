# Many Time Pad

Instructor notes.
Provide 1-2 hours to crack the Many Time Pad.
See the [Many_Time_Pad_Activity.md](./Many_Time_Pad_Activity.md) file for instructions for students.

## Helpful Hints

If (ond only if) students are struggling, make these suggestion (in order) over time:

1. The messages (and if you didn't change it this time, the key) are in english, unicode encoded.
1. The inverse of XOR is XOR itself.
   XOR is a bitwise operation, and characters for messages are Unicode encoded 4 byte codes (represented by 2 hex code characters: `hex(a) = 62).
1. XOR the ciphertexts together, and consider what happens when a space is XORed with a character in [a-zA-Z].
1. Consider Crib dragging: https://samwho.dev/blog/toying-with-cryptography-crib-dragging/ .
   (This is basically a full solution to find the key, and should unblock almost everyone)

## Potential Solution

For instructor use only: https://joshorndorff.github.io/DanBonehsCryptoExercises/crib-dragging/ is a tool used to perform crib dragging interactively.

## How to Configure new messages

- Generate cyphertext for some text: [PRIVATE-generate_ciphertexts-many-time-pad.py](./PRIVATE-generate_ciphertexts-many-time-pad.py)
- Update the instructions to use the correct cyphertext: [Many_Time_Pad_Activity.md](./Many_Time_Pad_Activity.md)
