---
title: Light Clients and Bridges
description: Light clients and bridges used in blockchains, for Web3 engineers
---

# Light Clients

TODO

---

# Bridges

---

## Transport

Bridges are transport layers between independent consensus systems.

---

## Forms

Bridges come in many forms.

- "On-chain light client" bridges place some logic on one chain that can verify the state of another
- Collateral-based bridges trust entities on one chain to hold assets as collateral for those on another

---

## Game Theory

There is also quite a large game theoretic design space with bridges.

They often have their own participants separate from the participants in either consensus system.

---

## Trust Implications

Two systems that are bridged have different notions of finality.

- How should the bridge handle messages if one chain is attacked?
- What happens to bridged assets if the _bridge_ is attacked?

Notes:

(Optionally) Engage class in brief discussion

---

## Using Bridges

- Best Case: Don't.
- OK Case: A bridge with some consensus verification means
- Worst Case: Collateral based bridges.

---
