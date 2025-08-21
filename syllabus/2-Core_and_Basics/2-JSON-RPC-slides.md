---
title: JSON-RPC Spec
description: JSON-RPC Spec
---

# JSON-RPC Spec

---

# JSON-RPC Spec

## What you will learn:

- What is JSON-RPC (v2)<!-- .element: class="fragment" -->
  - Its stateless design<!-- .element: class="fragment" -->
  - Conventions to make it stateful<!-- .element: class="fragment" -->
- Do nodes have to expose JSON-RPC APIs?<!-- .element: class="fragment" -->
- Legacy JSON-RPC APIs (Hands-on)<!-- .element: class="fragment" -->
  - Summary of probems<!-- .element: class="fragment" -->
- Modern JSON-RPC: Introduction and goals<!-- .element: class="fragment" -->
  - Overview of the new JSON-RPCs APIs<!-- .element: class="fragment" -->
  - Hands-on examples <!-- .element: class="fragment" -->

---

## JSON-RPC 2.0

- JSON-RPC is a stateless, transport agnostic, light-weight remote procedure call (RPC) protocol.<!-- .element: class="fragment" -->
- Defines basic data-structures and the rules around their processing. It is transport agnostic.<!-- .element: class="fragment" -->

---

## JSON-RPC 2.0 - Request Object

- Must have an `id` property

<!-- .element: class="fragment" -->

- Must have a `method`

<!-- .element: class="fragment" -->

- May have `params`

<!-- .element: class="fragment" -->

---

## JSON-RPC 2.0 - Notification Object

- Like `Request` but without an `id`

---

## JSON-RPC 2.0 - Response

- Must have an `id`

<!-- .element: class="fragment" -->

- Must have either a `result` or an `error`

<!-- .element: class="fragment" -->

- If it has an `error`, it must have the following properties:
  - `code`: number that indicates the type of error. Error codes from -32768 to -32000 are reserved for pre-defined errors.
  - `message`: string providing a short description.
  - `data`: optional data structure with additional information.

<!-- .element: class="fragment" -->

---

## JSON-RPC 2.0 Examples

https://www.jsonrpc.org/specification#examples

---

## JSON-RPC 2.0 For stateful connections and subscriptions

- The client acts as a server<!-- .element: class="fragment" -->
- The server acts as a client<!-- .element: class="fragment" -->
- Use opaque ids that are only relevant in the context of their connection<!-- .element: class="fragment" -->

---

# Do nodes have to expose JSON-RPC APIs?

- Different kinds of nodes, and differet kinds of usages:<!-- .element: class="fragment" -->

  - Light-node<!-- .element: class="fragment" -->
  - Full-node<!-- .element: class="fragment" -->
  - Archive-node<!-- .element: class="fragment" -->

- Different kinds of consumers<!-- .element: class="fragment" -->
  - Dapps<!-- .element: class="fragment" -->
  - Indexers<!-- .element: class="fragment" -->
  - Wallets<!-- .element: class="fragment" -->
  - Bridges<!-- .element: class="fragment" -->
  - Monitoring systems, node-admins, etc<!-- .element: class="fragment" -->

**Notes:**

- First ask users to list the different kinds of nodes that they are aware off.
- Ask them about different kinds of usages for different kinds of nodes.
- Explain why it makes sense to have different groups of functions depending on the type of node.

---

# Legacy JSON-RPC APIs (Hands-on)

**Notes:**

- Start by googling "polkadot json-rpc" and explaining why what we got into the results is very sad.
- Go to: https://polkadot.js.org/docs/substrate/rpc/
  - Briefly explain why that is a complete cluster-fuck
  - From the terminal try the `getHeader` API, show the returned payload and discuss the implications
  - From the terminal: use `substribeAllHeads` and explain the implications from a load-balancer stand-point
  - From the terminal: `state_call` vs `state_getMetadata`, why does `state_getMetadata` exist?

---

## Summary of problems

- Poorly documented<!-- .element: class="fragment" -->
- "Leaky": many endpoins are tightly-coupled to specific runtimes<!-- .element: class="fragment" -->
- Bad defaults: uses the best-block as the default block<!-- .element: class="fragment" -->
- Not "load-balancer-friendly": the server can't signal the client that a subscription is done.<!-- .element: class="fragment" -->
- Not "light-client friendly"<!-- .element: class="fragment" -->
- Can't be versioned<!-- .element: class="fragment" -->

**Notes:**

---

# Modern JSON-RPC: Introduction and goals

- Properly speced out<!-- .element: class="fragment" -->
- Different clients/consumers => different groups of functions<!-- .element: class="fragment" -->
- Minimal surface API<!-- .element: class="fragment" -->
- Load-balancer friendly: subscriptions can be terminated by the server<!-- .element: class="fragment" -->
- DoS attacks resilience<!-- .element: class="fragment" -->

**Notes:**

- Grouping functions using prefixes with versioning ensures clear evolution and compatibility.
- Each node type supports functions based on its capabilities, ensuring efficient and relevant operations.
- Clients should always use `rpc_methods` to verify supported functions on a given node.

---

## Overview of the new JSON-RPCs APIs

- https://paritytech.github.io/json-rpc-interface-spec/

**Notes:**

- Go the the spec and explain the contracts between the old and the new:
- Properly speced, errors well defined, cancelable subscriptions, etc
- Versions and groups of functions, etc
- ChainHead deep-dive

---

## Hands-on examples

**Notes:**

- Go to the termminal and walk them through the prepared scripts
