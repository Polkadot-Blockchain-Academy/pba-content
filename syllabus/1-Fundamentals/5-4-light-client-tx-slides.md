---
title: LCTX 4 - Submit and Track Transaction
description: End to End Transaction Flow with a Light Client
---

# Light Client Transaction

#### Part 4: Submit and Track Transaction

---

## Submitting the Extrinsic

- The light client will then call the RPC `author_submitAndWatchExtrinsic` over a WebSocket connection.
- This will accept the extrinsic payload AND return a subscription id to track the progress of the extrinsic.
- From that point forward, the node will push updates back to the light client, with the `SubscriptionId`.

See: https://github.com/paritytech/jsonrpsee

---

## Stages of the Subscription

```rust
pub enum TransactionStatus<Hash, BlockHash> {
  /// Transaction is part of the future queue.
  Future,
  /// Transaction is part of the ready queue.
  Ready,
  /// The transaction has been broadcast to the given peers.
  Broadcast(Vec<String>),
  /// Transaction has been included in block with given hash
  /// at the given position.
  InBlock((BlockHash, TxIndex)),
  /// Transaction has been finalized by a finality-gadget, e.g. GRANDPA.
  Finalized((BlockHash, TxIndex)),
  /// Transaction is no longer valid in the current state.
  Invalid,
  /* -- more variants not included -- */
}
```

---

## Purpose of Transaction Pool Validation

Moving transactions from one list to the other.

<diagram class="mermaid" style="display: flex; width: 80%">
graph LR
    W --"😇 ⏳"--> F["⏰ Future"]
    W --"😇 ⌛️"--> R["✅ Ready"]
    W["🤠 Wild West"] --"😈"--> T["🗑️ Invalid"]
</diagram>

---

### Transaction Validation Logic

- Transaction validity is exclusively outside of the transaction pool, and is **100% determined by the Runtime**.
- Transaction validation should be **cheap** to perform.
- Transaction pool is entirely an **offchain operation**.
  - No state change

Once these checks pass, an subscription update is sent with the `Ready` or `Future` status.

Notes:

Important, must pause and ask!

- Why is it from the runtime? because the transaction format is opaque and the node doesn't even know what to do with it.
- Why does it have to be cheap? wild west, unpaid, DoS!
- Pesky question: but be aware that from the runtime's perspective, the node could be malicious. The runtime cannot trust the node to obey.
  ** THE RUNTIME MUST RE-VALIDATE TRANSACTIONS LATER in block building and import as well **

---

## Basic Transaction Pool Validation

The transaction pool first does low cost, fast to validate operations coming from transaction extension logic:

- Is the included signature valid?
- Is the provided nonce valid?
- Is the provided era still ongoing?
- Does the user have enough balance to pay the transaction fee?
- Is the transaction targeting the correct chain?
- etc...

---

### The `validate_transaction` Function

The runtime API.

```rust[1-100|6]
impl TaggedTransactionQueue<Block> for Runtime {
  fn validate_transaction(
    source: TransactionSource,
    tx: <Block  as BlockT>::Extrinsic,
    block_hash: <Block as BlockT>::Hash,
  ) -> TransactionValidity {
    ..
  }
}
```

---

### Representation of `TransactionValidity`

```rust[1-100|5-6|8-9|11-12|14-100|1-100]
pub type TransactionValidity = Result<ValidTransaction, TransactionValidityError>;

/// This is going into `Ready` or `Future`
pub struct ValidTransaction {
  /// If in "Ready", what is the priority?
  pub priority: u64,

  /// For how long is this validity?
  pub longevity: u64,

  /// Should be propagate it?
  pub propagate: bool,

  /// Does this require any other tag to be present in ready?
  ///
  /// This determines "Ready" or "Future".
  pub requires: Vec<Tag>,
  /// Does this provide any tags?
  pub provides: Vec<Tag>,
}

type Tag = Vec<u8>
```

---

### Banning Invalid Transactions

- If a transaction is discovered to be invalid, **its hash** is banned for a fixed duration of time.
- Default in substrate is `Duration::from_secs(60 * 30)`, can be configured via CLI.

Notes:

See: https://github.com/paritytech/substrate/pull/11786

we probably also ban the peer who sent us that transaction? but have to learn.

---

## Transaction Gossipping (Broadcast)

- If a transaction is determined to be valid, the node starts gossipping it to peers.
- Another subscription update is sent with `Broadcast`, and a list of peers it was sent to.
- The basic validation and gossiping process will repeat throughout the network.

---

## Transaction Pool Ordering

In the `ValidTransaction` struct contains parameters `provides` and `requires`, which allows us to:

- Specify if a transaction is "Ready" or "Future".
- Determine what transactions should go before others.

Transactions will not be `Ready` until another transaction `provides` what it `requires`, if anything.

Note: it essentially forms a graph.

Order mostly matters within the ready pool. I am not sure if the code maintains an order in `future` as well. In any
case, not a big big deal.

---

### Nonce

The nonce helps with transaction ordering and more!

1. Ordering
2. Replay protection
3. Double spend protection

---

### `Provides` and `Requires` Examples

- A transaction in Bitcoin-like chain will:

  - `provide` generated UTXOs.
  - `require` UTXOs it is still awaiting for.

- A transaction in account-based chain will:
  - `provide` a `(sender, nonce)` as one tag.
  - `require` `(sender, nonce - 1)`.

---

### Transaction Ordering: Quiz Time (1)

<pba-cols>
<pba-col>

```
(
  A,
  provides: vec![],
  requires: vec![]
)
```

</pba-col>

<pba-col>
<table>
<thead>
  <tr>
    <th>Ready</th>
    <th>Future</th>
  </tr>
</thead>
<tbody class="fragment">
  <tr>
    <td>
    <pre>(A, pr: vec![], rq: vec![])</pre>
    </td>
    <td></td>
  </tr>
</tbody>
</table>
</pba-col>

</pba-cols>

---

### Transaction Ordering: Quiz Time (2)

<pba-cols>
<pba-col>

```
(
  B,
  provides: vec![2],
  requires: vec![1]
)
```

</pba-col>

<pba-col>
<table>
<thead>
  <tr>
    <th>Ready</th>
    <th>Future</th>
  </tr>
</thead>
<tbody class="fragment">
  <tr>
    <td>
      <pre>(A, pr: vec![], rq: vec![])</pre>
    </td>
    <td>
      <pre>(B, pr: vec![2], rq: vec![1])</pre>
    </td>
  </tr>
</tbody>
</table>
</pba-col>

</pba-cols>

---

### Transaction Ordering: Quiz Time (3)

<pba-cols>
<pba-col>

```
(
  C,
  provides: vec![3],
  requires: vec![2]
)
```

</pba-col>

<pba-col>
<table>
<thead>
  <tr>
    <th>Ready</th>
    <th>Future</th>
  </tr>
</thead>
<tbody class="fragment">
  <tr>
    <td>
      <pre>(A, pr: vec![], rq: vec![])</pre>
    </td>
    <td>
      <pre>(B, pr: vec![2], rq: vec![1])</pre>
    </td>
  </tr>
  <tr>
    <td>
    </td>
    <td>
      <pre>(C, pr: vec![3], rq: vec![2])</pre>
    </td>
  </tr>
</tbody>
</table>
</pba-col>

</pba-cols>

---

### Transaction Ordering: Quiz Time (4)

<pba-cols>
<pba-col>

```
(
  D,
  provides: vec![1],
  requires: vec![]
)
```

</pba-col>

<pba-col>
<table>
<thead>
  <tr>
    <th>Ready</th>
    <th>Future</th>
  </tr>
</thead>
<tbody class="fragment">
  <tr>
    <td>
      <pre>(A, pr: vec![], rq: vec![])</pre>
    </td>
    <td>
    </td>
  </tr>
  <tr>
    <td>
      <pre>(D, pr: vec![1], rq: vec![])</pre>
    </td>
    <td>
    </td>
  </tr>
  <tr>
    <td>
      <pre>(B, pr: vec![2], rq: vec![1])</pre>
    </td>
    <td>
    </td>
  </tr>
  <tr>
    <td>
      <pre>(C, pr: vec![3], rq: vec![2])</pre>
    </td>
    <td>
    </td>
  </tr>
</tbody>
</table>
</pba-col>

</pba-cols>

Note: The oder in this slide matters and it is top to bottom.

---

### Further Transaction Ordering: `priority` and more...

- From the `Ready` pool, when all requirements are met, then `priority` dictates the order.
- Priority is assigned by runtime logic, and can be controlled by runtime engineers!
- Beyond priority, there are further tie breakers:
  1. ttl: shortest `longevity` goes first
  2. time in the queue: longest to have waited goes first

Note:

https://github.com/paritytech/polkadot-sdk/blob/bc53b9a03a742f8b658806a01a7bf853cb9a86cd/substrate/client/transaction-pool/src/graph/ready.rs#L146

---

## Block Inclusion

The transaction will make its way into a block producing node, and eventually included in the new block.

---

<!-- .slide: data-background-color="#4A2439" -->

# Questions
