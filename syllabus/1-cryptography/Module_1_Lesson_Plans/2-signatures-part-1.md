# Lesson 2: Digital Signatures (Part 1)

## Core Ideas to Convey

1. Basic operations:

   - Sign a message
     - _Note that the contents of what you are signing are critical_
   - Sign something _unique_ ideally, so these cannot be replayed(?)
     - Revoke a key

2. Basic end-user considerations (for end users)

   - Key management (wallets)
   - Popper operation considerations for personal security

3. Maybe some signature schemes from lesson 3, so people can start coding.

4. Sr-25519 and Blake-2-256 (standard APIs)
   - Learners can actually start working with these on Day 1(!)
   - Working with the Substrate abstraction on top of that
   - No need for details here

<!-- TODO: The instructor should have a Rust console open and should be showing how a signature is made during this section. -->

## Activities and Exercises

- **[EVCXR](https://github.com/google/evcxr)** can be installed
  - Google Repl for Rust
  - Console where you can type Rust declarations and snippets
- VSCode or another standardized editor
