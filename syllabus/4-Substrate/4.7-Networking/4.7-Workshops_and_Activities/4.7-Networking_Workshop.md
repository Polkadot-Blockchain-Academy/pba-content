## Workshop

Two workshop ideas for this lecture:

1. Implement a polite gossip protocol

- Build something on top of `sc-network` directly.
- Use the code from the `network/protocol` and `network/bridge` or from `finality-grandpa` in the Substrate repo to help craft minimal examples.

2. Create a protocol version upgrade example

Show how to use `sc-network` to configure peers to attempt to connect on a protocol but negotiate a fallback protocol (typically the old version).
Show how to detect when the peer is connected on the fallback protocol and trigger different behavior as a result.
