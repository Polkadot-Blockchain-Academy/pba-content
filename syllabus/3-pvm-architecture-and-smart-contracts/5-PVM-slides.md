---
title: PolkaVM
description: PVM architecture
duration: 30 minutes
---

# PolkaVM

- From the past (Wasm) to the future (PVM)
- "the next generation virtual machine for Polkadot and PolkaJam"

---

# Wasm upsights

- Open standard
- Mature and widely adopted
- Many production-grade implementation available

---

# Wasm shortcomings

## Determinism

- Wasm is not actually deterministic
  - Unbounded stack
  - Example: Register allocation

---

# Wasm shortcomings

## Determinism

What is the problem with the below snippet?

Hint: Think about the differences between how function arguments are passed in stack vs. register machines.


```wasm
(func (param i32) (param i32) (param i32) (param i32) (param i32) (param i32) (param i32) (param i32) (param i32)
    local.get 0
    local.get 1
    ;; ...
    local.get 8

    call $other
)
```

Note: 

https://forum.polkadot.network/t/deterministic-pvf-executor/4204
https://hackmd.io/@Ww6uNnIISmqGwXufqPYPOw/SklLYwb-T

---

# Wasm shortcomings

## Determinism


```wasm
(func (param i32) (param i32) (param i32) (param i32) (param i32) (param i32) (param i32) (param i32) (param i32)
    local.get 0
    local.get 1
    ;; ...
    ;;
    ;; general purpose registers and ABI of the target platform and architecture?
    ;;
    ;; x86_64:  6 argument registers (rdi, rsi, rcx, rdx, r8, r9)
    ;; aarch64: 8 argument registers (r0..r7)
    ;; x86_64:  may eventually spill to stack
    ;; x86_64:  will eventually stack overflow
    ;; aarch64: probably fine?
    ;;
    ;; => indeterministic behavior!
    ;;
    local.get 8

    call $other
)
```

---

# Wasm shortcomings

## Compilation

---

# Wasm shortcomings

## More problems

- Complexity
- Open but our request were turned down in the past
- `wasmtime` sandboxing security issues in past

---

# PVM Architecture

## RISC-V

<img src="./img/RISC-V.svg" style="width: 100%">

---

# PVM Architecture

## JIT

## Compiler

## Sandboxing

---

# PVM Architecture

## ELF linker

---
# PVM Architecture

## RISC-V

---
# PVM Architecture

## RISC-V

---

# PVM Architecture

## RISC-V

---

# PVM Benchmarks

---

# PVM Demo

## Doom (maybe on-chain with JAM)

---