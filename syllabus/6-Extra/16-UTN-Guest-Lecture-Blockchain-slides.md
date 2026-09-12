---
title: La arquitectura de blockchains modernas
description: Charla invitada para la UTN (Universidad Tecnológica Nacional, Argentina) por la Polkadot Blockchain Academy.
duration: 2 hours
owner: TODO
---

<!-- .slide: data-background-image="../../assets/img/0-Shared/bg/PBA_Background.png" data-background-size="cover" -->

# La arquitectura de blockchains modernas

TODO: título, tu nombre, branding de PBA + UTN

---

## El punto de partida

TODO: ustedes ya escriben smart contracts — un contrato es un huésped en la máquina de otro: corre bajo reglas que no puede cambiar. Hoy vemos cómo se construye la máquina. Agenda por actos.

Diagram idea: un contrato chiquito (ícono de documento) viviendo adentro de una máquina grande etiquetada "blockchain", con un candado en las reglas de la máquina

---

# Acto 1

## Anatomía de una blockchain

---

## La máquina de estados

TODO: definir estado, extrinsics y la state transition function (STF): `estado_nuevo = STF(estado, extrinsic)`; la cadena como historial de transiciones que cualquiera puede re-ejecutar y verificar

Diagram idea: caja "Estado S" + flecha etiquetada "extrinsic" pasando por una caja "STF" hacia otra caja "Estado S'"; abajo, dos personas recomputando y llegando al mismo resultado con tilde

---

## Las capas de la máquina

TODO: presentar las 4 capas — networking, consenso, runtime (protocolo), aplicación — y qué vive en cada una; la charla las recorre de abajo hacia arriba

Diagram idea: stack vertical de 4 capas etiquetadas "Networking / Consenso / Runtime / Aplicación" con ejemplos chicos al costado de cada una

---

## Networking

TODO: gossip P2P, mempool, propagación de bloques y transacciones entre pares sin servidor central

Diagram idea: malla de nodos conectados entre sí con flechas de gossip propagando una transacción que nace en un nodo

---

## Consenso

TODO: qué le garantiza el consenso al runtime: un único historial ordenado y finalidad; mencionar PoW/PoS/BFT solo a nivel conceptual

Diagram idea: varios nodos proponiendo historiales distintos y una flecha de "consenso" seleccionando un único historial final con tilde

---

## Aplicación

TODO: la capa que la audiencia ya conoce: contratos (y pallets) viviendo dentro del runtime, usando las garantías de las capas de abajo

Diagram idea: zoom al tope del stack donde varios contratos/pallets chiquitos se apoyan sobre la caja "Runtime"

---

## ¿Cómo se cambian las reglas de esta máquina?

TODO: slide puente sin contenido — dejar la pregunta plantada para abrir el Acto 2

Diagram idea: la máquina del stack con un signo de pregunta grande sobre la capa "Runtime" y un engranaje roto

---

# Acto 2

## El runtime: por qué separarlo del nodo

---

## El problema de ingeniería

TODO: si las reglas viven embebidas en el software del nodo, actualizarlas exige coordinar a miles de operadores: hard forks, clientes desactualizados, riesgo de split de la cadena

Diagram idea: línea de tiempo con una actualización v1→v2 donde la mitad de los nodos actualiza y la otra mitad no, y la cadena se bifurca en dos

---

## La solución: nodo vs runtime

TODO: separar el nodo/cliente (networking, consenso, DB) del runtime (la STF, la lógica de la aplicación) como artefactos independientes

Diagram idea: una caja de nodo dividida horizontalmente — abajo "Nodo: networking, consenso, DB", arriba "Runtime: STF / lógica" — con flechas entre ellas etiquetadas "runtime APIs / host functions"

---

## La analogía de la consola

TODO: el nodo es la consola, el runtime es el cartucho intercambiable: el mismo hardware corre cualquier juego

Diagram idea: consola etiquetada "nodo" con slot de cartucho; un cartucho "runtime A" siendo cambiado por otro "runtime B"

---

## El runtime vive on-chain

TODO: el runtime se guarda como Wasm en el propio estado de la cadena — la cadena transporta sus propias reglas

Diagram idea: cadena de bloques con una celda de storage resaltada etiquetada "Runtime (Wasm)" y una flecha desde esa celda hacia la caja runtime del nodo

---

## Forkless upgrades y el nodo "oblivious"

TODO: la gobernanza on-chain actualiza la lógica sin que los operadores toquen nada; el nodo ni siquiera necesita saber qué lógica corre

Diagram idea: línea de tiempo donde el cartucho/runtime se actualiza v1→v2→v3 por votos de gobernanza mientras la consola/nodo de abajo queda intacta

---

## Substrate, conceptualmente

TODO: el framework del nodo: networking (libp2p), motores de consenso, base de datos, executor Wasm — todo lo que no es la lógica de la cadena

Diagram idea: la caja "nodo" del stack explotada en sus piezas internas (networking, consenso, DB, executor) con la etiqueta "Substrate"

---

## FRAME, conceptualmente

TODO: el runtime se compone de pallets: módulos con storage, calls, events, hooks; el runtime es la suma de sus pallets

Diagram idea: la caja "runtime" como un rompecabezas armado con piezas etiquetadas "Balances", "Staking", "tu pallet"

---

## Pallet vs smart contract

TODO: la comparación clave para esta audiencia — qué se puede expresar en cada uno, quién paga por los bugs, cómo se actualiza cada uno, límites de la sandbox del contrato

Diagram idea: tabla de dos columnas "Smart contract" / "Pallet" con filas "qué puede tocar", "cómo se actualiza", "quién gobierna las reglas" y marcas simples

---

## ¿Cuándo graduar un contrato a pallet?

TODO: el camino natural: prototipar como contrato, y cuando necesitás más control (lógica de consenso, fees propios, performance) migrar a pallet

Diagram idea: flecha de graduación de izquierda a derecha: "contrato" → "pallet" → con umbrales etiquetados "más control", "más responsabilidad"

---

## ¿Y si una sola máquina no alcanza?

TODO: slide puente — una máquina bien diseñada sigue teniendo un techo; abre el Acto 3

Diagram idea: la máquina del stack con un techo físico encima y una flecha de tráfico creciente chocando contra el techo

---

# Acto 3

## Escalar: de una cadena a muchas

---

## El techo monolítico

TODO: cada nodo re-ejecuta todo; subir el throughput exige más hardware, y más hardware excluye validadores — el trilema como trade-off de ingeniería (throughput / seguridad / descentralización)

Diagram idea: triángulo con vértices "throughput", "seguridad", "descentralización" y un punto adentro mostrando que solo podés acercarte a dos

---

## La división modular

TODO: separar la blockchain en capas especializadas: ejecución / settlement / consenso / data availability — cada una escala por su cuenta

Diagram idea: la torre monolítica cortada en 4 capas horizontales (ejecución / settlement / consenso / DA) con flechas mostrándolas como piezas intercambiables

---

## Rollups: la respuesta que ya conocen

TODO: ejecución afuera de la L1, settlement y DA prestados de la L1; optimistic vs ZK en una línea cada uno

Diagram idea: bloques de ejecución del rollup arriba, una flecha con el "resumen + prueba" bajando hacia la L1 etiquetada "settlement + DA"

---

## El costo de los rollups

TODO: fragmentación de liquidez y componibilidad entre rollups, y los puentes como punto débil de seguridad

Diagram idea: varios rollups como islas sobre la misma L1, con puentes frágiles entre ellos y un candado roto en uno

---

## El problema del bootstrap de seguridad

TODO: una cadena nueva tiene market cap chico y token especulativo → es barata de atacar → lanzar una cadena soberana segura es casi imposible al inicio

Diagram idea: una cadena bebé con un escudo diminuto siendo mirada por un atacante grande, al lado de una cadena vieja con escudo enorme

---

## Seguridad compartida

TODO: muchas cadenas compartiendo un solo presupuesto económico de seguridad; mencionar que no todas las formas de "seguridad compartida" son equivalentes

Diagram idea: varias cadenas chicas aportando sus escuditos a un escudo gigante compartido que las cubre a todas

---

## Cómo lo hace Polkadot

TODO: validadores de la relay chain asignados al azar re-validan los bloques de las parachains; approval checking a alto nivel; un bloque inválido se rechaza y el collator se penaliza

Diagram idea: conjunto central de validadores con flechas hacia bloques de parachains que están re-chequeando, y un bloque inválido marcado con una X roja rechazado

---

## Cores

TODO: el core como unidad atómica de ejecución asegurada — el recurso real que Polkadot vende

Diagram idea: la relay chain como una grilla de slots tipo núcleos de CPU, cada uno ejecutando una parachain distinta

---

## Rollups vs parachains

TODO: la misma idea modular con distinta historia de seguridad y componibilidad: seguridad prestada de una L1 de propósito general vs seguridad nativa compartida; puentes vs mensajería nativa (XCM)

Diagram idea: dos columnas "Rollup (Ethereum)" / "Parachain (Polkadot)" con filas "de dónde viene la seguridad", "cómo se comunican entre pares", "quién define el entorno de ejecución"

---

## ¿Es este modelo el final?

TODO: slide puente — relay chain + parachains funciona, pero es rígido: cadenas enteras por core, cadencia fija; abre el Acto 4

Diagram idea: la grilla de cores con cada parachain encadenada a su slot, y un signo de pregunta: "¿y si solo quiero correr un programa chico?"

---

# Acto 4

## La frontera: coretime, PolkaVM, JAM

---

## Agile coretime

TODO: el blockspace como producto: comprar cores en bulk o on-demand según necesidad, en lugar de subastas largas; qué es una región de coretime

Diagram idea: la grilla de cores con etiquetas de precio: algunos slots marcados "bulk" y uno marcado "on-demand" con un carrito de compras

---

## PolkaVM

TODO: una VM basada en RISC-V con performance casi nativa; reemplaza Wasm/EVM como motor de ejecución de Polkadot; Solidity compila a PVM vía pallet-revive

Diagram idea: gráfico de barras o velocímetro comparando EVM / Wasm / PolkaVM (RISC-V), con PolkaVM cerca de "velocidad nativa"

---

## JAM: un computador descentralizado

TODO: JAM generaliza el modelo: de relay chain + parachains a un pool de cores corriendo servicios arbitrarios y permissionless — una cadena es solo un tipo más de servicio

Diagram idea: flecha de transformación — izquierda "relay chain + parachains", derecha "JAM: pool de cores" corriendo servicios heterogéneos (una cadena, un rollup, una app)

---

## Qué cambia para quien construye

TODO: el camino se acorta: en lugar de lanzar una cadena, deployás código — contratos hoy, servicios mañana; la escalera de opciones se completa

Diagram idea: camino de izquierda a derecha con tres escalones: "contrato" → "parachain" → "servicio en JAM", cada uno más cerca del core

---

# Acto 5

## Demo en vivo

---

## Demo A: tour de un runtime

TODO: checklist en pantalla — abrir Polkadot.js Apps sobre Paseo testnet; mostrar la lista de pallets; explorar storage items de un pallet; enviar un extrinsic; leer los eventos emitidos

TODO: preparar red local de backup por si falla la conexión

---

## Demo B: coretime y parachains en vivo

TODO: checklist en pantalla — mostrar los cores y su asignación; ver una venta de coretime (Coretime chain o UI tipo RegionX); mostrar una parachain produciendo bloques en tiempo real

TODO: preparar capturas de backup por si falla la conexión

---

# Cierre

---

## Recap: la escalera

TODO: contrato → pallet → parachain → core: cada nivel te da más control sobre la máquina a cambio de más responsabilidad; cerrar el arco con la pregunta inicial — ahora saben cómo se construye la máquina

Diagram idea: la escalera de 4 escalones (contrato / pallet / parachain / core) con una flecha ascendente etiquetada "más control, más responsabilidad"

---

<!-- .slide: data-background-color="#000000" -->

# Preguntas
