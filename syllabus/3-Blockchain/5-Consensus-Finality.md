TODO split this out of existing consensus slides.

---

Add details about how PBFT works, consider an activity where we play it manually maybe tendermint style first.

---

TODO Grandpa board game!! Try to invent it.

Split students into groups of 5

One is responsible for block authoring, and is not playing grandpa. Remaining 4 are playing grandpa, this allows for finality without unanimity. Each of the four has their own board representing their view of the tree. Author adds blocks to the boards. Must add it to each board. There is latency between when each node receives it. This is good.

Players cast votes (prevotes and precommits) for blocks and post those votes on each otehrs boards. Again there is latency and discrepency in ordering. Again this is good.

Consider introducing rules to increase latency, fail to broadcast some messages, etc
