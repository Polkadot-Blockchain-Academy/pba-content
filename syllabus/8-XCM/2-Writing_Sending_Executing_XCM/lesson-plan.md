## Lesson 2: Writing, Sending, and Executing XCM

> {Likely} Instructor: Keith, plus another RT Eng.?

### Why is this topic Important?

### Prerequisite Knowledge or Skills

- Relative file/directory path understanding ( .. , ../.. , etc.)
- SCALE codec

### Learning Outcome

- XCM construction concretely (examples) are understood.
- Understand the XCM instruction actually gets executed.
    - Cover https://github.com/paritytech/xcm-format#4-basic-xcvm-operation in depth
- Practically write XCM business logic


### Learning Objectives

- Construct appropriate XCM, given a set of business requirements.
- MultiLocations and MultiAssets understood very well, practically.

### Core Ideas to Convey


- MultiLocations
    - Re-anchoring
      - Multilocation → relative path. The destination needs to be able to re-express from it’s perspective where the message originated from. Example : Parent/Parent need to be expressed from the context/perspective of this location that could be a contract on a parachain for example
- MultiAssets
- XCM instructions/programs detailed
- Default executor logic in detail

### Activities and Exercises