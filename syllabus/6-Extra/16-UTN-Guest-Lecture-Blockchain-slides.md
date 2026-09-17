---
title: La arquitectura de blockchains modernas
description: Charla invitada para la UTN (Universidad Tecnológica Nacional, Argentina) por la Polkadot Blockchain Academy.
duration: 2 hours
---

<!-- .slide: data-background-image="../../assets/img/0-Shared/bg/PBA_Background.png" data-background-size="cover" -->

# La arquitectura de blockchains modernas

Francisco Aguirre, UTN Setiembre 2026

Notes:

Desarrollador en Parity Technologies.
Ingeniero en computación de la UdelaR de Montevideo, Uruguay.
Ex alumno de la primera PBA en Cambridge 2022.

---v

<img style="width: 900px; height: auto" src="./img/UTN-lecture/UTN lecture 1.excalidraw.png" alt="UTN lecture diagram 1" />

Notes:

Vamos a empezar desde lo más básico, qué es una blockchain, generalizar, modularizar, escalar y abstraer para obtener arquitecturas cada vez más complejas.

---

<!-- .slide: data-background-color="#000000" -->

# Capítulo 1

## Blockchain

Notes:

Así que empezamos por el comienzo: vamos a repasar qué es una blockchain.

---v

### Estructura de datos

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 2.excalidraw.png" alt="UTN lecture diagram 2" />

Notes:

Una blockchain es una estructura de datos muy simple en la que los bloques tienen una serie de operaciones (o transacciones) y cada bloque referencia al anterior mediante su hash.
No es una linea recta sino que presenta forks.

---v

<h3 style="margin-top: 150px; margin-bottom: -225px;">Sistema decentralizado</h3>
<div style="display: flex; justify-content: center; align-items: center; gap: 40px; transform: scale(50%);">
  <div style="display: flex; flex-direction: column;">
    <img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/PC.png" alt="UTN lecture diagram 4" />
    <img style="max-width: 800px; max-height: 550px;" src="./img/UTN-lecture/UTN lecture 2.excalidraw.png" alt="UTN lecture diagram 2" />
  </div>
  <span style="font-size: 80px">↔</span>
  <div style="display: flex; flex-direction: column;">
    <img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/PC.png" alt="UTN lecture diagram 4" />
    <img style="max-width: 800px; max-height: 550px;" src="./img/UTN-lecture/UTN lecture 2.excalidraw.png" alt="UTN lecture diagram 2" />
  </div>
  <span style="font-size: 80px">↔</span>
  <div style="display: flex; flex-direction: column;">
    <img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/PC.png" alt="UTN lecture diagram 4" />
    <img style="max-width: 800px; max-height: 550px;" src="./img/UTN-lecture/UTN lecture 2.excalidraw.png" alt="UTN lecture diagram 2" />
  </div>
</div>

Notes:

Cuando decimos blockchain usualmente no hablamos solo de la estructura de datos sino de un sistema decentralizado.

---v

### Decentralizado ≠ Distribuído

Notes:

Cuando decimos "sistema decentralizado", no nos referimos únicamente a el aspecto distribuído del mismo.
En un sistema distribuído, los nodos pueden todos pertenecer a la misma compañía, haciendolo centralizado.

---v

<div style="transform: scale(150%);">

### Capas

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 4.excalidraw.png" alt="UTN lecture diagram 4" />

Notes:

Estos sistemas tienen varias capas.
La capa de red es la más baja que suele ser una librería como libp2p.
La capa de consenso es como todas las máquinas de la red deciden quien va a producir el siguiente bloque.
La capa de protocolo es la que define las operaciones que van en los bloques y como se procesan.
La capa de aplicacion permite a los usuarios interactuar con el sistema.

---v

### Maquina de estados

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 5.excalidraw.png" alt="UTN lecture diagram 5" />

Notes:

Al final del dia, uno de estos sistemas es una maquina de estados replicada en varias maquinas.
Es un ambiente adversarial, en el que todas las maquinas son propiedad de distintos individuos y empresas.
En el corazon de todo eso, en la capa de protocolo, esta la funcion de transicion de estado que toma el estado anterior y un bloque y devuelve el nuevo estado.

---v

<!-- .slide: data-background-color="#000000" -->

## Bitcoin

Notes:

Vamos a empezar hablando de Bitcoin dado que es la blockchain mas simple.

---v

### Función de transición de estado

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 6.excalidraw.png" alt="UTN lecture diagram 6" />

Notes:

La funcion de transicion de estado de Bitcoin es simplemente hacer transferencias de UTXOs.

---v

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 7.excalidraw.png" alt="UTN lecture diagram 7" />

Notes:

El bloque, de manera simplificada, tiene las transferencias y el header la raiz del merkle tree de todas las transacciones del bloque.

---v

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 8.excalidraw.png" alt="UTN lecture diagram 8" />

Notes:

Estas son las implementaciones de cada capa en Bitcoin.

---v

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 9.excalidraw.png" alt="UTN lecture diagram 9" />

Notes:

Podemos permitir que los usuarios programen esta maquina decentralizada?

---

<!-- .slide: data-background-color="#000000" -->

# Capítulo 2

## Generalizar

---v

<!-- .slide: data-background-color="#000000" -->

## Ethereum

Notes:

Vamos a hablar de Ethereum, EVM y Smart Contracts, que generalizan la STF y la hacen mucho mas util.

---v

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 10.excalidraw.png" alt="UTN lecture diagram 10" />

---v

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 11.excalidraw.png" alt="UTN lecture diagram 11" />

---v

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 12.excalidraw.png" alt="UTN lecture diagram 12" />

---v

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 13.excalidraw.png" alt="UTN lecture diagram 13" />

---v

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 14.excalidraw.png" alt="UTN lecture diagram 14" />

---

<!-- .slide: data-background-color="#000000" -->

# Capítulo 3

## Modularizar

---v

<!-- .slide: data-background-color="#000000" -->

## Substrate

---v

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 15.excalidraw.png" alt="UTN lecture diagram 15" />

---v

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 16.excalidraw.png" alt="UTN lecture diagram 16" />

---v

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 17.excalidraw.png" alt="UTN lecture diagram 17" />

---v

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 18.excalidraw.png" alt="UTN lecture diagram 18" />

---v

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 19.excalidraw.png" alt="UTN lecture diagram 19" />

---v

<img style="max-width: 1000px; max-height: 700px" src="./img/UTN-lecture/UTN lecture 20.excalidraw.png" alt="UTN lecture diagram 20" />

---v

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 21.excalidraw.png" alt="UTN lecture diagram 21" />

---v

<!-- .slide: data-background-color="#000000" -->

## FRAME

---v

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 22.excalidraw.png" alt="UTN lecture diagram 22" />

---v

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 23.excalidraw.png" alt="UTN lecture diagram 23" />

---v

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 24.excalidraw.png" alt="UTN lecture diagram 24" />

---v

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 25.excalidraw.png" alt="UTN lecture diagram 25" />

---v

## Hooks

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 26.excalidraw.png" alt="UTN lecture diagram 26" />

---v

## Transaction extensions

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 27.excalidraw.png" alt="UTN lecture diagram 27" />

---v

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 28.excalidraw.png" alt="UTN lecture diagram 28" />

---v

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 29.excalidraw.png" alt="UTN lecture diagram 29" />

---v

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 30.excalidraw.png" alt="UTN lecture diagram 30" />

---v

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 31.excalidraw.png" alt="UTN lecture diagram 31" />

---v

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 32.excalidraw.png" alt="UTN lecture diagram 32" />

---v

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 33.excalidraw.png" alt="UTN lecture diagram 33" />

---v

<!-- .slide: data-background-color="#000000" -->

## DEMO

---

<!-- .slide: data-background-color="#000000" -->

# Capítulo 4

## Escalar

---v

### Vertical scaling

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 34.excalidraw.png" alt="UTN lecture diagram 34" />

---v

### Rollups

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 35.excalidraw.png" alt="UTN lecture diagram 35" />

---v

### Sovereign chains

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 36.excalidraw.png" alt="UTN lecture diagram 36" />

---v

### Shared security

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 37.excalidraw.png" alt="UTN lecture diagram 37" />

---v

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 38.excalidraw.png" alt="UTN lecture diagram 38" />

---v

### Panopticon

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/panopticon.webp" alt="Panopticon" />

---v

<!-- .slide: data-background-color="#000000" -->

## ELVES

---v

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 39.excalidraw.png" alt="UTN lecture diagram 39" />

---v

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 40.excalidraw.png" alt="UTN lecture diagram 40" />

---v

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 41.excalidraw.png" alt="UTN lecture diagram 41" />

---v

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 42.excalidraw.png" alt="UTN lecture diagram 42" />

---v

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 43.excalidraw.png" alt="UTN lecture diagram 43" />

---v

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture - 1.excalidraw.png" alt="UTN lecture diagram - 1" />

---v

<!-- .slide: data-background-color="#000000" -->

## 1. Collation

---v

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 44.excalidraw.png" alt="UTN lecture diagram 44" />

---v

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 45.excalidraw.png" alt="UTN lecture diagram 45" />

---v

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 46.excalidraw.png" alt="UTN lecture diagram 46" />

---v

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 47.excalidraw.png" alt="UTN lecture diagram 47" />

---v

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 48.excalidraw.png" alt="UTN lecture diagram 48" />

---v

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 49.excalidraw.png" alt="UTN lecture diagram 49" />

---v

<!-- .slide: data-background-color="#000000" -->

## 2. Backing

---v

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 50.excalidraw.png" alt="UTN lecture diagram 50" />

---v

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 51.excalidraw.png" alt="UTN lecture diagram 51" />

---v

<!-- .slide: data-background-color="#000000" -->

## 3. Data availability

---v

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 52.excalidraw.png" alt="UTN lecture diagram 52" />

---v

<!-- .slide: data-background-color="#000000" -->

## 4. Approval checking

---v

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 53.excalidraw.png" alt="UTN lecture diagram 53" />

---v

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 54.excalidraw.png" alt="UTN lecture diagram 54" />

---v

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 55.excalidraw.png" alt="UTN lecture diagram 55" />

---v

<!-- .slide: data-background-color="#000000" -->

## 5. Disputes

---v

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 56.excalidraw.png" alt="UTN lecture diagram 56" />

---v

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 57.excalidraw.png" alt="UTN lecture diagram 57" />

---v

<!-- .slide: data-background-color="#000000" -->

## DEMO

---

<!-- .slide: data-background-color="#000000" -->

# Capítulo 5

## Abstraer

---v

<!-- .slide: data-background-color="#000000" -->

## JAM

---v

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/jam-logo.png" alt="UTN lecture diagram 58" />

---v

<div style="display: flex; justify-content: center; align-items: center; gap: 40px; margin: 300px;">
  <img style="height: 220px; width: auto" src="./img/UTN-lecture/wasm-logo.png" alt="WASM logo" />
  <span style="font-size: 80px">→</span>
  <img style="height: 220px; width: auto" src="./img/UTN-lecture/risc-v-logo.png" alt="Risc-V logo" />
</div>

---v

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 58.excalidraw.png" alt="UTN lecture diagram 58" />

---v

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 59.excalidraw.png" alt="UTN lecture diagram 59" />

---v

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 60.excalidraw.png" alt="UTN lecture diagram 60" />

---v

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 61.excalidraw.png" alt="UTN lecture diagram 61" />

---v

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 62.excalidraw.png" alt="UTN lecture diagram 62" />

---v

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/jam-doom-2.png" alt="UTN lecture diagram 62" />

https://www.youtube.com/watch?v=hJcw5FMSjQs

---v

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/jam-doom.png" alt="UTN lecture diagram 62" />

---v

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 63.excalidraw.png" alt="UTN lecture diagram 63" />

---v

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 64.excalidraw.png" alt="UTN lecture diagram 64" />

---v

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 65.excalidraw.png" alt="UTN lecture diagram 65" />

---v

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 66.excalidraw.png" alt="UTN lecture diagram 66" />

---v

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 67.excalidraw.png" alt="UTN lecture diagram 67" />

---v

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/UTN lecture 68.excalidraw.png" alt="UTN lecture diagram 68" />

---v

<img style="max-width: 800px; max-height: 550px" src="./img/UTN-lecture/jam-computer.png" alt="UTN lecture diagram 68" />

---v

<!-- .slide: data-background-color="#000000" -->

## ¿Preguntas?

---v

<!-- .slide: data-background-color="#000000" -->

## Gracias
