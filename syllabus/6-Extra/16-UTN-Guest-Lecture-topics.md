# Charla UTN — Lista de temas

**Título:** La arquitectura de blockchains modernas
**Evento:** Charla invitada para la UTN (Universidad Tecnológica Nacional, Argentina)
**Duración:** 2 horas
**Audiencia:** estudiantes de CS con conocimiento de blockchain, principalmente del lado de smart contracts
**Template de slides:** [`16-UTN-Guest-Lecture-Blockchain-slides.md`](./16-UTN-Guest-Lecture-Blockchain-slides.md)

## Estructura narrativa

Una sola historia con cuatro movimientos — **generalizar → modularizar → escalar → abstraer** — contada sobre la lente de las capas: una base de **datos (DB, transversal a todo)** con **networking / consenso / runtime / aplicación** arriba. **La meta de la charla: que entiendan qué es un pallet y en qué se diferencia de un smart contract**, anclado en la analogía kernel space / user space (precompiles = syscalls).

Decisiones de diseño pedagógico:

- La **modularización se enseña antes que el escalado** (orden didáctico), con honestidad histórica: la separación nodo/runtime se imaginó para validar shards con ELVES, y los forkless upgrades vinieron de yapa. El círculo cierra en el capítulo 4.
- Rollups / Solana / Cosmos son un **interludio breve** cuando aparece el problema de escala (capítulo 2), con un contraste de una línea cada uno en el capítulo 4 — no un capítulo propio.
- La capa de **DB/estado merklizado** se introduce en Bitcoin y paga dos veces: light clients/SPV ahí mismo, y el PoV (proof of validity) en el capítulo 4 — es la segunda pieza que hace posible ELVES.
- La distinción kernel/user space es **universal** (Ethereum: la EVM es el kernel y los precompiles son syscalls literales); en Polkadot el kernel se compone de pallets y cambia por gobernanza que se auto-ejecuta — te ahorrás el hard fork, no la gobernanza. La graduación máxima: parachain propia = el runtime es tuyo, sin sandbox.

## Intro (~10 min)

| #   | Tema                                                                                  | Material de origen en este repo                               |
| --- | ------------------------------------------------------------------------------------- | ------------------------------------------------------------- |
| 1   | Título                                                                                | —                                                             |
| 2   | El punto de partida: el contrato como huésped; la meta: entender pallets vs contratos | —                                                             |
| 3   | La lente: DB transversal + 4 capas + los 4 movimientos                                | `syllabus/3-Substrate/01-Introduction-to-Substrate-slides.md` |

## Capítulo 1 — Bitcoin: la máquina específica (~10 min)

| #   | Tema                                                                                                | Material de origen en este repo                                                                                                                                        |
| --- | --------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 4   | Bitcoin como máquina de estados (UTXOs, STF mínima)                                                 | `syllabus/1-Fundamentals/01-What-is-a-Blockchain-slides.md` ("Bitcoin As an Authority"); `previous_syllabus/prev_campuses/3-Blockchain/4-Accounts_and_UTXOs-slides.md` |
| 5   | ¿Dónde vive todo esto? DB + historial merklizado (Bitcoin NO merkliza el estado); SPV/light clients | `syllabus/3-Substrate/04-Merklized-Storage-slides.md`; `syllabus/1-Fundamentals/03-Light-Client-Verifiable-Storage-Proofs-slides.md`                                   |
| 6   | Las capas en Bitcoin (DB / P2P / PoW / Script / transferencias)                                     | `previous_syllabus/prev_campuses/3-Blockchain/2-P2P_Networks-slides.md`, `5-Consensus-Authoring-slides.md`                                                             |
| 7   | Puente: ¿y si la STF ejecutara cualquier programa?                                                  | —                                                                                                                                                                      |

## Capítulo 2 — Generalizar la computación: Ethereum (~15 min)

| #   | Tema                                                                                                           | Material de origen en este repo                                                                                                                              |
| --- | -------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 8   | La EVM: la STF programable; plantar kernel/user space universal (precompiles de Ethereum = syscalls literales) | `syllabus/2-Polkadot/01-Introduction-to-Polkadot-slides.md` ("General Monolithic")                                                                           |
| 9   | También se generalizan los datos: Ethereum merkliza el ESTADO (state root) → pruebas de storage; hook al cap 4 | `syllabus/3-Substrate/04-Merklized-Storage-slides.md`                                                                                                        |
| 10  | Lo que NO cambió: dos problemas (reglas en el cliente / todos re-ejecutan)                                     | `syllabus/3-Substrate/03-Wasm-Meta-Protocol-slides.md` ("BUT WAIT A MINUTE")                                                                                 |
| 11  | Interludio: tres respuestas rápidas al problema de escala (rollups, Solana, Cosmos)                            | `syllabus/2-Polkadot/01-Introduction-to-Polkadot-slides.md` ("Blockchain Landscape"); `previous_syllabus/prev_campuses/7-Polkadot/Blockchain_Scaling_Part_2` |
| 12  | …y sus issues en una línea cada uno                                                                            | —                                                                                                                                                            |
| 13  | Puente: Polkadot toma otro camino — abrir la máquina                                                           | —                                                                                                                                                            |

## Capítulo 3 — Modularizar la máquina: Substrate/FRAME (~30 min, el centro)

| #   | Tema                                                                                                                                                                              | Material de origen en este repo                                                                                                 |
| --- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------- |
| 14  | Separar nodo y runtime; consola y cartucho                                                                                                                                        | `syllabus/1-Fundamentals/06-Light-Client-State-Transition-Function-slides.md` ("The Client vs Runtime", "Game Console Analogy") |
| 15  | El runtime vive on-chain (Wasm)                                                                                                                                                   | `syllabus/3-Substrate/03-Wasm-Meta-Protocol-slides.md` (Part 1)                                                                 |
| 16  | Lo que habilita: forkless upgrades + validadores que ejecutan cualquier STF                                                                                                       | Mismo archivo ("Radical Upgradeability", "Oblivious Node")                                                                      |
| 17  | Dato histórico: se diseñó para ELVES; los upgrades vinieron de yapa                                                                                                               | `previous_syllabus/prev_campuses/7-Polkadot/2-shared-security-slides.md` ("Wasm", "A Polkadot Validator Node")                  |
| 18  | Substrate: el framework del nodo                                                                                                                                                  | `syllabus/3-Substrate/01-Introduction-to-Substrate-slides.md`                                                                   |
| 19  | FRAME: el runtime se compone de pallets                                                                                                                                           | `syllabus/4-FRAME/01-Introduction-to-FRAME-slides.md`                                                                           |
| 20  | Anatomía de un pallet: storage, calls, events, hooks                                                                                                                              | Mismo archivo ("Building Blocks of Pallets")                                                                                    |
| 21  | La analogía del SO: kernel/user es universal; en Polkadot el kernel se compone de pallets y cambia por gobernanza que se auto-ejecuta — te ahorrás el hard fork, no la gobernanza | Síntesis propia (ancla conceptual de la charla)                                                                                 |
| 22  | El kernel de Polkadot: Governance, Staking, Assets (fungibles y NFTs), Multisig (espejo de Safe)                                                                                  | `syllabus/1-Fundamentals/07-The-Polkadot-Stack-slides.md` ("Asset Hub"); `syllabus/2-Polkadot/02-Staking-slides.md`             |
| 23  | Superpoderes del kernel space: transaction extensions (incl. transacciones sponsoreadas), hooks, fees propios, offchain workers                                                   | `syllabus/4-FRAME/14-Transaction-Extensions-slides.md`; `syllabus/4-FRAME/09-Pallet-Hooks-slides.md`                            |
| 24  | Precompiles = syscalls: contratos llamando pallets                                                                                                                                | `syllabus/1-Fundamentals/07-The-Polkadot-Stack-slides.md` ("pallet-revive"); material de precompiles de pallet-revive           |
| 25  | **Pallet vs smart contract — la slide central; espejo Safe (contrato) vs pallet Multisig (kernel)**                                                                               | `syllabus/4-FRAME/01-Introduction-to-FRAME-slides.md` ("Pallet vs. Smart Contract", "Smart Contract vs. Pallet")                |
| 26  | Cuándo graduar un contrato a pallet — y la graduación máxima: parachain propia = el runtime es tuyo, sin sandbox                                                                  | Mismo archivo ("When to Reach for a Pallet", "Scaling a Successful Contract")                                                   |
| 27  | Puente: una máquina modular sigue siendo UNA máquina                                                                                                                              | —                                                                                                                               |

## Capítulo 4 — Escalar: ELVES y seguridad compartida (~20 min)

| #   | Tema                                                                                                         | Material de origen en este repo                                                                                                            |
| --- | ------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------ |
| 28  | Retomar el techo / trilema                                                                                   | `syllabus/2-Polkadot/06-Execution-Sharding-slides.md` ("Scalability of Blockchains")                                                       |
| 29  | Muchas máquinas en paralelo (parachains)                                                                     | `syllabus/2-Polkadot/04-State-Sharding-slides.md`                                                                                          |
| 30  | ¿Quién vigila cada shard? + bootstrap de seguridad                                                           | `previous_syllabus/prev_campuses/7-Polkadot/2-shared-security-slides.md` ("The Bootstrapping Problem")                                     |
| 31  | Cierra el círculo: el runtime portable hace posible validar shards                                           | Mismo archivo ("Wasm", "A Polkadot Validator Node")                                                                                        |
| 32  | ELVES: asignación aleatoria, backing, approval checking, PoV (callback al estado merklizado de Ethereum), DA | `syllabus/2-Polkadot/06-Execution-Sharding-slides.md` ("ELVES", "Approval Checking"); `syllabus/2-Polkadot/05-Data-Availability-slides.md` |
| 33  | Lo que compra: seguridad compartida, cores; contraste con rollups/Solana/Cosmos                              | `syllabus/2-Polkadot/08-Agile-Coretime-slides.md`; síntesis del interludio                                                                 |

## Capítulo 5 — La frontera: JAM (~10 min)

| #   | Tema                                         | Material de origen en este repo                                                                                                                           |
| --- | -------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 34  | Límites del modelo parachain; agile coretime | `syllabus/2-Polkadot/08-Agile-Coretime-slides.md`                                                                                                         |
| 35  | JAM: cores + servicios; PolkaVM (RISC-V)     | `previous_syllabus/bali - 2025/6-JAM/13-demystifying-jam-slides.md`; `17-JAM-CoreVM-slides.md`; `syllabus/1-Fundamentals/07-The-Polkadot-Stack-slides.md` |
| 36  | Qué cambia para quien construye              | —                                                                                                                                                         |

## Demo en vivo (~15–20 min, flexible)

| #   | Tema                                                                                            | Material de origen en este repo                                                                                                          |
| --- | ----------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------- |
| 37  | Demo A: tour de un runtime — pallets (Governance, Staking, Assets), storage, extrinsic, eventos | `syllabus/2-Polkadot/09-Polkadot-SDK-and-Tools-slides.md`; `syllabus/6-Extra/13-playground-demo-slides.md`; Polkadot.js Apps sobre Paseo |
| 38  | Demo B: cores, venta de coretime, parachain en vivo                                             | Coretime chain en Polkadot.js Apps o UI tipo RegionX                                                                                     |

## Cierre (~5 min)

| #   | Tema                                              | Material de origen en este repo |
| --- | ------------------------------------------------- | ------------------------------- |
| 39  | Recap: los cuatro movimientos + kernel/user space | —                               |
| 40  | Preguntas                                         | —                               |
