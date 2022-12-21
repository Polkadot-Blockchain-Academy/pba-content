# Accounting Models & User Abstractions on Blockchains
* Andrew Burger
* Parity, Integritee. Yay blockchain and things
---
## Review what we have spoken about so far
* Cryptography (Signatures, Hash functions, Hash based Data Structures)
* Economics/Game Theory
* Blockchain structure (The overall blockchain primitive, a hash list)
---
## Where do we go from here?
* We have some base elements ideas and concepts now lets put them together into something cool..
Notes:
    TODO: Show some generic picture of assembling things
---
## What do we want to talk about today?
* Now that we have this structured decentralized tamper proof state machine lets think of ways we can formulate a state and a state transition in terms of representing users 
Notes:
    TODO: Insert generic picture of someone thinking
---
## Let us think of two different paradigms:
* A.) We craft a model in which the system acts as a global and trustless verifier
Notes:
    TODO: Insert picture here which shows some money being sent from person A to person B and the System box is verifying whether the money I am attempting to spend is mine and the result that I am specifying to happen is valid 
---
## Paradigm A How would we do this?
* 1.) You have some uniquely identifiable piece of data with a signature attached(A hash which signifies some id of a spendable thing + a signature saying I can spend that thing) In more computer science terms we have some data that can be altered only by a specific entity so we want to provide proof we can alter it. 
* 2.) You have some value assosciated with this piece of data and an owner(Value + Pubkey) Some data which can be altered along with a key stating who can alter it.
* Thing 1 we can refer to as an input
* Thing 2 we can refer to as an output
---
##
---
### Make a request for verification by this system (A transition)(We described somewhat of the `State`) Now lets talk about how to transition the state using this new verification model.
* We described somewhat of the `State` Now lets talk about how to transition the state using this new verification model.
---
## Next paradigm..
* B.) We craft a model in which the system acts as a computer and by submitting some input it will determine the result from my input.
Notes:
    TODO: Insert some picture here which shows some money being sent from person A to person B but instead person A just signs a message saying to send to person B and the system determines what the updated output result or state will be
---
