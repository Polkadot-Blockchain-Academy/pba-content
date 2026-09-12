# Charla UTN — Lista de temas

**Título:** La arquitectura de blockchains modernas
**Evento:** Charla invitada para la UTN (Universidad Tecnológica Nacional, Argentina)
**Duración:** 2 horas
**Audiencia:** estudiantes de CS con conocimiento de blockchain, principalmente del lado de smart contracts
**Template de slides:** [`16-UTN-Guest-Lecture-Blockchain-slides.md`](./16-UTN-Guest-Lecture-Blockchain-slides.md)

## Estructura narrativa

Hilo conductor: _un smart contract es un huésped en la máquina de otro — hoy vemos cómo se construye la máquina_. Cada acto abre con un problema técnico concreto y cierra con la pregunta que abre el siguiente.

## Acto 0 — Intro (~5 min)

| #   | Tema                                                            | Material de origen en este repo |
| --- | --------------------------------------------------------------- | ------------------------------- |
| 1   | Título                                                          | —                               |
| 2   | El punto de partida: el contrato como huésped; agenda por actos | —                               |

## Acto 1 — Anatomía de una blockchain (~25 min)

| #   | Tema                                                    | Material de origen en este repo                                                            |
| --- | ------------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| 3   | La máquina de estados: STF, historial verificable       | `syllabus/1-Fundamentals/06-Light-Client-State-Transition-Function-slides.md`              |
| 4   | Las capas: networking / consenso / runtime / aplicación | `syllabus/3-Substrate/01-Introduction-to-Substrate-slides.md`                              |
| 5   | Networking: gossip P2P, mempool, propagación            | Mismo archivo                                                                              |
| 6   | Consenso: orden y finalidad para el runtime             | `previous_syllabus/prev_campuses/3-Blockchain/8-Consensus_Finality-slides.md` (conceptual) |
| 7   | Aplicación: contratos y pallets dentro del runtime      | — (ancla con lo que ya conocen)                                                            |
| 8   | Puente: ¿cómo se cambian las reglas?                    | —                                                                                          |

## Acto 2 — El runtime: por qué separarlo del nodo (~25 min)

| #   | Tema                                                             | Material de origen en este repo                                                               |
| --- | ---------------------------------------------------------------- | --------------------------------------------------------------------------------------------- |
| 9   | El problema: reglas en el software del nodo → hard forks, splits | `syllabus/3-Substrate/03-Wasm-Meta-Protocol-slides.md` ("BUT WAIT A MINUTE")                  |
| 10  | Nodo vs runtime como artefactos separados                        | `syllabus/1-Fundamentals/06-...State-Transition-Function-slides.md` ("The Client vs Runtime") |
| 11  | Analogía de la consola y el cartucho                             | Mismo archivo ("Game Console Analogy")                                                        |
| 12  | El runtime vive on-chain (Wasm)                                  | `syllabus/3-Substrate/03-Wasm-Meta-Protocol-slides.md` (Part 1)                               |
| 13  | Forkless upgrades, nodo "oblivious"                              | Mismo archivo ("Radical Upgradeability", "Oblivious Node")                                    |
| 14  | Substrate conceptualmente                                        | `syllabus/3-Substrate/01-Introduction-to-Substrate-slides.md`                                 |
| 15  | FRAME conceptualmente: pallets                                   | `syllabus/4-FRAME/01-Introduction-to-FRAME-slides.md`                                         |
| 16  | Pallet vs smart contract — slide clave                           | Mismo archivo ("Pallet vs. Smart Contract", "Smart Contract vs. Pallet")                      |
| 17  | Cuándo graduar un contrato a pallet                              | Mismo archivo ("When to Reach for a Pallet", "Scaling a Successful Contract")                 |
| 18  | Puente: ¿y si una sola máquina no alcanza?                       | —                                                                                             |

## Acto 3 — Escalar: de una cadena a muchas (~25 min)

| #   | Tema                                                        | Material de origen en este repo                                                                                                                                            |
| --- | ----------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 19  | El techo monolítico / trilema                               | `syllabus/2-Polkadot/01-Introduction-to-Polkadot-slides.md` ("Blockchain Landscape"); `syllabus/2-Polkadot/06-Execution-Sharding-slides.md` ("Scalability of Blockchains") |
| 20  | La división modular: ejecución / settlement / consenso / DA | `previous_syllabus/prev_campuses/7-Polkadot/Blockchain_Scaling_Part_2/Blockchain_Scaling-slides.md`                                                                        |
| 21  | Rollups: ejecución afuera, settlement + DA prestados        | Mismo archivo                                                                                                                                                              |
| 22  | El costo: fragmentación, puentes débiles                    | —                                                                                                                                                                          |
| 23  | El bootstrap de seguridad                                   | `previous_syllabus/prev_campuses/7-Polkadot/2-shared-security-slides.md` ("The Bootstrapping Problem")                                                                     |
| 24  | Seguridad compartida                                        | Mismo archivo ("Shared Security")                                                                                                                                          |
| 25  | Cómo lo hace Polkadot: re-validación                        | Mismo archivo ("Deep Dive Into Polkadot Shared Security"); `syllabus/2-Polkadot/06-Execution-Sharding-slides.md` (approval checking, alto nivel)                           |
| 26  | Cores: la unidad de ejecución asegurada                     | `syllabus/2-Polkadot/08-Agile-Coretime-slides.md`                                                                                                                          |
| 27  | Rollups vs parachains                                       | Síntesis de los anteriores                                                                                                                                                 |
| 28  | Puente: ¿es este modelo el final?                           | —                                                                                                                                                                          |

## Acto 4 — La frontera (~15 min)

| #   | Tema                                               | Material de origen en este repo                                                                                                                                   |
| --- | -------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 29  | Agile coretime: blockspace como producto           | `syllabus/2-Polkadot/08-Agile-Coretime-slides.md`                                                                                                                 |
| 30  | PolkaVM: RISC-V, velocidad casi nativa             | `previous_syllabus/prev_campuses/7-Polkadot/2-shared-security-slides.md` ("PolkaVM"); `syllabus/1-Fundamentals/07-The-Polkadot-Stack-slides.md` ("pallet-revive") |
| 31  | JAM: computador descentralizado, cores + servicios | `previous_syllabus/bali - 2025/6-JAM/13-demystifying-jam-slides.md`; `17-JAM-CoreVM-slides.md`                                                                    |
| 32  | Qué cambia para quien construye                    | `syllabus/1-Fundamentals/07-The-Polkadot-Stack-slides.md`                                                                                                         |

## Acto 5 — Demo en vivo (~20 min)

| #   | Tema                                                      | Material de origen en este repo                                                                            |
| --- | --------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------- |
| 33  | Demo A: tour de un runtime (Polkadot.js Apps sobre Paseo) | `syllabus/2-Polkadot/09-Polkadot-SDK-and-Tools-slides.md`; `syllabus/6-Extra/13-playground-demo-slides.md` |
| 34  | Demo B: cores, venta de coretime, parachain en vivo       | Coretime chain en Polkadot.js Apps o UI tipo RegionX                                                       |

## Cierre (~5 min)

| #   | Tema                                                    | Material de origen en este repo |
| --- | ------------------------------------------------------- | ------------------------------- |
| 35  | Recap: la escalera contrato → pallet → parachain → core | —                               |
| 36  | Preguntas                                               | —                               |
