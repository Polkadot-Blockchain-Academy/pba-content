# PBP Lisbon 2026 - Project Resources

A curated starting set for your project phase. Not exhaustive. If you find
something better than what's listed here, open a PR - this file is meant to
evolve with the cohort.

Sections mirror the paths in [`project-guidelines.md`](./project-guidelines.md) §3:
pick a backend, pick a frontend, deploy on Bulletin Chain + DotNS.

---

## Starter templates

- [`polkadot-stack-template`](https://github.com/shawntabrizi/polkadot-stack-template) - opinionated backend + web frontend starter from Shawn.
- [`polkadot-sdk-minimal-template`](https://github.com/paritytech/polkadot-sdk-minimal-template) - the smallest viable Substrate runtime.
- [`pba-omni-node`](https://github.com/kianenigma/pba-omni-node) - generic node that runs your custom runtime without you writing node-side code.

## Backend: Substrate Pallet

- [`paritytech/polkadot-sdk`](https://github.com/paritytech/polkadot-sdk) - the monorepo: Substrate, FRAME, Cumulus, Polkadot.
- [Polkadot SDK docs](https://docs.polkadot.com/) - official docs portal for the SDK.
- [Substrate Stack Exchange](https://substrate.stackexchange.com/) - Q&A site, actively answered.
- [`frame-metadata`](https://github.com/paritytech/frame-metadata) - the on-chain metadata format.

## Backend: Solidity on EVM or PVM

- [Polkadot smart-contract tutorials](https://docs.polkadot.com/tutorials/smart-contracts/) - official walkthroughs for Solidity on Polkadot.
- [`paritytech/revive`](https://github.com/paritytech/revive) - Solidity → PVM compiler.
- [Foundry](https://book.getfoundry.sh/) - modern Solidity dev/test toolchain.
- [Hardhat](https://hardhat.org/) - alternative Solidity toolchain with rich JS/TS integration.

## Backend: Rust contracts on PVM (RevX)

> Bleeding-edge path - see guidelines §4. RevX is in open beta. Budget time
> for sharp edges and plan for a strong bug report as a legitimate deliverable.
> Note: this path uses **RevX**, not ink!.

- [RevX](https://revx.dev/) - Rust smart-contract IDE for Polkadot.

## Frontend: chain interaction libraries

- [PAPI (`papi.how`)](https://papi.how/) - modern, type-safe TypeScript client for Polkadot chains.
- [PAPI metadata explorer](https://dev.papi.how/explorer) - interactive explorer for any chain's metadata.
- [polkadot.js API](https://polkadot.js.org/docs/) - older TS client, very widely documented.
- [`subxt`](https://github.com/paritytech/subxt) ([book](https://docs.rs/subxt/latest/subxt/index.html)) - type-safe Rust client for Substrate chains.

## Frontend: Web / TUI / MCP

- [polkadot.js Apps](https://polkadot.js.org/apps/) - the reference web UX for any Substrate chain.
- [`ratatui`](https://ratatui.rs/) - Rust TUI framework.
- [Model Context Protocol (MCP) docs](https://modelcontextprotocol.io/) - spec and SDKs for writing an MCP server.

## Deployment: Bulletin Chain + DotNS (mandatory)

- [`polkadot-bulletin-chain`](https://github.com/paritytech/polkadot-bulletin-chain) - the Bulletin Chain repo; the README links out to all the current docs.
- [`dot.li`](https://dot.li/) - DotNS link shortener.

## Local dev & testing

- [`zombienet`](https://github.com/paritytech/zombienet) / [`zombienet-sdk`](https://github.com/paritytech/zombienet-sdk) - spin up ephemeral multi-node networks for integration tests.
- [Zombienet VS Code extension](https://github.com/paritytech/zombienet-vscode-extension) - IDE support for writing Zombienet specs.
- [Chopsticks](https://docs.polkadot.com/develop/toolkit/parachains/fork-chains/chopsticks/) - fork a live chain locally and test against real state.

## Testnets, RPCs, explorers

- [Paseo](https://paseo.network/) - community testnet for Polkadot.
- [telemetry.polkadot.io](https://telemetry.polkadot.io/) - live network health dashboard.

## Reference & community

- [Polkadot Wiki](https://wiki.polkadot.com/) - community explainers across the whole stack.
- [Polkadot Forum](https://forum.polkadot.network/) - official discussion forum.
- [Substrate Stack Exchange](https://substrate.stackexchange.com/) - Q&A.
