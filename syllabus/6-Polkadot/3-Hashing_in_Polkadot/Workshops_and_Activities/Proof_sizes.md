## Proof sizes

TODO consider proposing a rustling version of the jupyter note book with cells being test case:
jupyter is rather tricky to use:

- run on /tmp -> need clear manually sometime as there is compilation artifact.
- only the last cell can really be retried, otherwhise need to recompile from scratch.

Starting with a runtime module scaffold.

1. for each block use block number and writes 1 time `world` at key `hello`.

Look at proof size and compact proof size and proof after compression of 100 first blocks.

2. for each block use block number and writes `world` at key `hello ++ u32be of block_nb`.

Look at proof size and compact proof size and proof after compression of 100 first blocks.

3. merge both writes and execute 10 blocks

4. Get similar values for {10, 100} block size chain and reading {1, 5, 10} random values from it.
   These read proofs are what you get when reading from runtime or querying an rpc.

5. Compare result and see that they match expectation from lesson

## Quizz if time

TODO slides with individuals question

-> TODO rephrase so these looks more related to the lesson. + when saying state-machine point out it involves cumulus.

- State machine does cache changes, and can access state through some key value cache layer.
  Considering that the key value cache layer is being shared between block execution (cache from block N-1 is accessible to block N as long as there was no reorg),
  can I?

a - use cache for block synch
b - use cache for proof check
c - use cache for proof building
d - use cache for rpc

a - d

b - we access state stored in the proof, accessing cache would means that the proof could be incomplete, so the check is wrong (the proof may require external data).
c - in this case if cache of a previous block is accessed, the database trie state is not queried and the value proof is not registered. But if cache access where registered
in state machine and proof build over it afterward, it can be use but then we have a local caching of accessed value, and it is equivalent to caching in the state machine.
d - surely as long as the rpc do not cover b or c.

- State machine alows iterating on the underlying trie structure. What is true? [AI]

a - The api `next_key` is quite restrictive because it is easier to handle changed value.
b - The api `next_key` is quite restrictive because of host function design.
c - Caching is implemented there.
d - Involve additional cost for state machine.
e - Using a patricia merke trie is relevant.

a? - b? - c? - d - e

a - it is certainly easier, but it also relate to general host function design that can get tricky for passing an iterator.
Still can consider caching last used trie iterator on host side and even a new host function that is host context related (like `iterator_from` and `nexts_from_current_iterator`).
But generally there is a preference for stateless host function design.

b - clearly host function are simple this way. But on the other hand the associated cost is quite high (on host call per items iterated on).
c - caching is not implemented, it was experimentally, but complex so at this time it is not in place.
d - underlying struct for change of state machine is a `BTreeMap` rather than a `HashMap` due to the need to implement iteration, it is not the same, for heavy block can be slower.
But that is marginal cost.
e - surely a ordered structure is relevant. Could still have implemented it less efficiently (eg linked list in values). But having a structure that allows to prove ordering is good.

- Merkle execution proofs size (eg merkle part of Pov) [A]

a - get bigger everytime we access state data.
b - get bigger for bigger values accessed
c - get bigger with a larger written values of a same written key.
d - get bigger with more written values of same length.
e - for a same value length can differ (eg querying different frame module)

b - e

a - everytime we access NEW state data.
c - d not the merkle proof being bigger but the size of the block (input of the proof). The value to writes
are part of the extrinsic or part of accessed state (in the second case it is the reading that increase proof size not the writing).
e - the trie is unbalanced and some module can have more trie depth and consequently different proof size.

- Proof verification model (cumulus/substrate pov case).

a - Block production and proof production can be done in a same execution.
b - Proof checking requires to fully execute the block.
c - Pvf wasm is the same as the parachain runtime.

a - b - c?

a - we can record the proof during block execution. But process could be separated. In fact if you consider that transaction are run prior this is not strictly true.

b - with merkle trie, we check state accesses that are define by the execution trace. The last state access being those done to calculate next block state root,
One can also have a block execution that do not record state root at the end of the execution so the proving will need to execute end of execution of block N-1 and block N up
to state root calculation. But the sum ends up being a full block execution. And the use case sounds wrong because checking proof will require two block as input instead of a single one.

If a different proof system was used, this statement would not have been true, this is specific to cumulus and merkle trie proof.

c - this is specific to cumulus, and make life easier. But considering polkadot design this is absolutelly false.
And it is interesting to notice that we can add queries to what is checked. For instance we can check some messenging state of the parachain in pvf but not in runtime, in this case creating the proof
will require adding manually these messenging queries: so as long as it is not a big overhead it is easier to do the queries in block execution and keep the same accesses between block synching and
pov. Meaning that PoV does not have to be strictly/only state transition proof. TODO this is giberish, rephrase: point is that pov can contains more than just state transition and multiple
design are doable.

- Execution model and sate access/proof. [A]

a - While synching blocks, I use the proof.
b - While producing blocks, I use the proof.
c - While running pvf, I use the parachain state.
d - While running pvf, I use the parachain state root.
e - On a light client, I use the chain state.
f - On a light client, I use the chain state root.

b? d f

a - we use the state from the already synched block.

b - we create the proof as the cost is marginal. Don't really have to

c - d - we use the state from a proof. (still require chain state root for block(s)).

- Other kind of proof (for instance proof that would have long production and quick checking).

a - We just execute block and check the resulting merkle trie root. This is a kind of state transition checking but we use the whole synched state.
b - No, as in a, but we also need to produce the proof:

- Previous question, but for any kind of proof system (%s/%r/I use /I can use /g).

a b d?

a - if checking the proof is faster we could attach a proof commitment to the header, download the proof and run it.
Then we would maintain state by adding the state delta similarily to the header.
So it make sense to run things faster. On substrate with a state cache, better save the network size of fetching the proof and associated delta.

b - creating the proof can be deffered, usually it is partly done when we run extrinsic individually. It all depends on the probability of failure of block execution.

cdef same as previously.

- substrate trie is radix 16 while it could just be a binary radix 2 patricia trie.[I]

a - for the binary trie proof would be smaller
b - radix 16 makes bigger nodes which results in worse storage access.

a -

a - true, radix 16 means that we attach more hash. The more value are queried, the lesser the difference will be (eg iterating on a range of value will tend to include all the radix 16 hashes).
b - Bigger node but worst storage, it makes less node to store with a still stay rather small size, and intend to be better.
radix 16 is worse for proof, better for storage, kind of a middle ground.
But truely, decoralating node storage from proof will be the best of both world.

- child trie.

a - are similar to the hash of the node of a branch of an unbalanced trie
b - could potentially hold different kind of storage.

a b

a - would say yes, but easier to implement, but in a sense having a trie attached to a node by including its root as value of the node (current child trie design) is close to attach a trie to a node.
b - yes as long as there is a way to produce proof and a short representation of the attached state (root).
