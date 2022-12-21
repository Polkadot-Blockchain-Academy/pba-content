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
## Is this our State? What is our state?
* A bunch of "outputs" (Some value or data which can be altered via a particular specified owner
Notes:
    TODO: Insert picture showing a table of hash values mapping to these "outputs"(Data + owner)
---
## So what is the notion of a User?
* A public key and all of the uniquely identifiable data which can be manipulated by that public key
---
### Make a request for verification by this system (A transition)(We described somewhat of the `State`) Now lets talk about how to transition the state using this new verification model.
* We described somewhat of the `State` Now lets talk about how to transition the State using this new verification model.
Notes:
    TODO: Show some image of something transitioning
---
### A "Transaction"
* Dont be confused it is essentially a request to change the state of the system.
* What does a Transaction look like? How do inputs map to outputs?
* A list of inputs + a list of outputs
---
### What were those input output thingies again?
Notes:
    TODO: Show picture of inputs and outputs in a transaction similar to UTXO frameless explain how new outputs are derived from previously specified outputs from the input 
---
## So what happens in a state transition or verification in this model?
* Inputs signal which outputs from the state will be consumed
* We must verify the signature given in the input with the corresponding specified outputs pubkey or owner 
* We must verify that the resulting state which has been provided is a valid one(We define what this is)
* In the case of money or just a raw value we can assume that you cannot take some piece of moneys worth 10 and create a list of new outputs which have more than 10 moneys
---
## UTXOS solved...
* Now whenever someone mentions UTXO's and the UTXO model you can now fundamentally know what actually is being referenced..(Hopefully!) 
Notes:
    TODO: Add some picture of a person cheering and happy
---
## Next paradigm..
* B.) We craft a model in which the system acts as a computer and by submitting some input it will determine the result from my input.
Notes:
---
## Well lets make things more intuitive!
* So instead of doing the computation ourselves and then submitting it to the system to verify...(i.e. understanding the present state and then computing the resulting state) 
* Why dont we just let the system hold our state information or data in the form of an "Account" and we submit requests and data to the system.
* This means we can let the system be our computer for us! 
---
## Well well what is state then in this system?
* Accounts -> Values
Notes:
    TODO: Show picture of a table mapping a pubkey to a value or data item(in its simplest form a value)
---
## Now that state is much easier to comprehend!!
* What about transitioning my state or account?
---
## State transitions a "Transaction"
* What does a transaction look like now?
Notes:
    TODO: Insert some picture here which shows some money being sent from person A to person B but instead person A just signs a message saying to send to person B and the system determines what the updated output result or state will be
---
## So what happens in a State transition in this computation model?
* We still verify but we verify less and determine more!
* Check the signature of the message or computational instruction given
* Verify that the account that is being requested to be modified is the signers account
* Perform the computation necessary given by the instruction and update the account accordingly!
---
## Accounts vs UTXO model
* Oh the fun begins..
Notes:
   TODO: Insert picture of Homer
---
## Scalability
Notes:
    TODO: Insert scalability differences
---
## Size(Storage)
---
## Sharding
---
## Privacy
---
## Smart contracts of general compute platforms
---
## Final differences
---
## At the end of the day it is just a state machine
Notes:
    TODO: Insert some fun picture and hint at how to agree on all of these now understood (Transactions) to come...
---
