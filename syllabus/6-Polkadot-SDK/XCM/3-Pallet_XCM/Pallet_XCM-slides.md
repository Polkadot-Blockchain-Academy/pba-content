---
title: XCM Pallet
description: Introduction to `pallet-xcm`, its interface and features implemented here.
duration: 1 hour
---

# XCM Pallet

## Response Handler

Version negotiation is just one example among many kinds of queries one chain can make to another.
Regardless of which kind of query was made, the response usually takes the form of a `QueryResponse` instruction.

---v

## Response Handler

We have talked about XCM being asymmetric, so why are there responses ?

---v

## Information Reporting

Every instruction used for information reporting contains `QueryResponseInfo`.

<pba-flex center>

```rust
pub struct QueryResponseInfo {
    pub destination: MultiLocation,
    pub query_id: QueryId,
    pub max_weight: Weight,
}
```

Notes:

All Information Reporting instructions contain a `QueryResponseInfo` struct, which contains information about the intended destination of the response, the ID of the query, and the maximum weight that the dispatchable call function can use.
The dispatchable call function is an optional operation that XCM author can specify, and is executed upon receiving the response, effectively acting as a lifecycle hook on response.

---v

## Information retrieval

<pba-flex center>

```rust
enum Instruction {
    // --snip--
    QueryResponse {
        query_id: QueryId,
        response: Response,
        max_weight: Weight,
        querier: Option<MultiLocation>,
    },
    // --snip--
}
```

Notes:

The above instruction is the one used for offering some requested information that the local system is expecting.
`querier` parameter should be checked to ensure that the system that requested the information matches with what is expected.
