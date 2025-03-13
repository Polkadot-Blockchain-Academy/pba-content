---
title: PolkaVM
description: PVM architecture
duration: 45 minutes
---

# PolkaVM

## From the past (Wasm) to the future (PVM)

> "the next generation virtual machine for Polkadot and PolkaJam"

-- Jan Bujak, author of the first PolkaVM implementation

---

# Wasm

Polkadot is abandoning Wasm: Here's why!

Note:

Click-baity introduction slide

---

# Wasm: The obvious choice?

- Open standard
- Mature and widely adopted
- Many production-grade implementation available
- Faster than EVM

---

# Wasm: The obvious choice **for blockchains**?

1. Indeterministic execution
2. Compiling Wasm to machine code

---

## Problem #1: Indeterministic execution

Executing Wasm code is not actually deterministic

<!-- .element: class="fragment" data-fragment-index="1" -->

Example: Unbounded stack

<!-- .element: class="fragment" data-fragment-index="2" -->

---

## Indeterministic execution

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

## Indeterministic execution

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

## Problem #2: Compiling Wasm to machine code

Wasm knows many "high level" control flow elements!

<!-- .element: class="fragment" data-fragment-index="1" -->

Wasm doesn't know about register allocation or target architecture specific optimizations!

<!-- .element: class="fragment" data-fragment-index="1" -->

Not ideal for us because compilation is expensive :(

<!-- .element: class="fragment" data-fragment-index="3" -->

---

### High level control flow

Compilers use (expensive) algorithms to:

<!-- .element: class="fragment" data-fragment-index="1" -->

- lower "high level" control flow statements
  - `loop`
  - `if`

<!-- .element: class="fragment" data-fragment-index="2" -->

- Into a bunch of "low level" control flow instructions
  - (conditional) branch instructions
  - location labels

<!-- .element: class="fragment" data-fragment-index="3" -->

---

### High level control flow

Let's use ChatGPT for a little experiment

---

### High level control flow

Prompt: _Write me fibonacci in webassembly text_

```wasm
  (func $fibonacci (param $n i32) (result i32)
    (local $a i32)
    (local $b i32)
    (local $temp i32)
    (local.set $a (i32.const 0))
    (local.set $b (i32.const 1))

    (loop $loop
      ;; if n <= 1, return n
      (if (i32.le_s (local.get $n) (i32.const 1))
        (then
          (return (local.get $n))
        )
      )
      ..
```

---

### High level control flow

Prompt: _Write me fibonacci in x86_64 assembly_

```asm
fibonacci:
    cmp rdi, 1          ; If n <= 1, return n
    jle .base_case
    mov rax, 0          ; a = 0
    mov rbx, 1          ; b = 1
    mov rcx, rdi        ; counter = n
.loop:
    add rax, rbx        ; temp = a + b
    xchg rax, rbx       ; a = b, b = temp
    dec rcx             ; counter--
    jg .loop            ; If counter > 0, repeat loop
    mov rax, rbx        ; Result is in b
    ret
.base_case:
    mov rax, rdi        ; Return n if n <= 1
    ret
```

---

<section>
  <h3>High level control flow</h3>
  <table>
  <thead>
    <tr>
      <th>Wasm</th>
      <th>x86_64</th>
    </tr>
  </thead>
    <tr>
      <td>
        <pre><code data-trim data-noescape>
        (loop $loop
          (if (i32.le_s (local.get $n) (i32.const 1))
            (then
              (return (local.get $n))
            )
          )
          ..
        )
        </code></pre>
      </td>
      <td>
        <pre><code data-trim data-noescape>
        fibonacci:
            cmp rdi, 1
            jle .base_case
            mov rax, 0
            mov rbx, 1
            mov rcx, rdi
        .loop:
            add rax, rbx
            xchg rax, rbx
            dec rcx
            jg .loop
            mov rax, rbx
            ret
        </code></pre>
      </td>
    </tr>
  </table>
</section>

---

<section>
  <h3>High level control flow</h3>
  <table>
  <thead>
    <tr>
      <th>Wasm</th>
      <th>x86_64</th>
    </tr>
  </thead>
    <tr>
      <td>
        <pre><code data-trim data-noescape>
        (loop $loop
          (if (i32.le_s (local.get $n) (i32.const 1))
        </code></pre>
      </td>
      <td>
        <pre><code data-trim data-noescape>
            cmp rdi, 1
            jle .base_case
            jg .loop
        </code></pre>
      </td>
    </tr>
  </table>
</section>

---

## Register allocation

Executing Wasm requires us to do register allocation

---

### Register allocation

> In compiler optimization, register allocation is the process of assigning local automatic variables and expression results to a limited number of processor registers.

https://en.wikipedia.org/wiki/Register_allocation

```wasm
(func (param i32) (param i32) (param i32) (param i32) (param i32) (param i32) (param i32) (param i32) (param i32)
    local.get 0
    local.get 1
    ;; ..
    local.get 8
    ;; Informally:
    ;;   Compiler needs to figure out which variable goes to which register
    ;;
    ;; For example adding two variables here requires them to be in
    ;; in registers on a real computer
)
```

---

### Register allocation

> **NP-Problem**
>
> Chaitin et al. showed that register allocation is an NP-complete problem.

https://en.wikipedia.org/wiki/Register_allocation#Common_problems_raised_in_register_allocation

---

### Register allocation

If you aren't familiar with the theory of computation, just read:

_Register allocation is a difficult problem_

---

## Compiling Wasm to machine code

- Wasm is almost a high level langauge (despite _assembly_ in the name)
- What compilers do is computationally expensive
- Can take literal hours on cheaper laptops
  - `polkadot-sdk`
  - `LLVM` framework
- There is a blog post somewhere, telling the story of a web dev rewriting everything to Wasm, only to end up with even greater load times :)

---

## Compiling Wasm to machine code

Not really a problem for long-living application like web apps

<!-- .element: class="fragment" data-fragment-index="1" -->

<img src="./img/pvm/this-is-fine.jpg">

<!-- .element: class="fragment" data-fragment-index="2" -->

---

## Compiling Wasm to machine code

- Every contract needs to be compiled on every node executing it!
- Transactions often involving multiple contracts amplifies the problem
- This doesn't look good from a theoretical perspective
- Caching?
  - Harder than it seems on first glance
  - Doesn't entirely solve the problem

---

## Wasm compilation: Take-aways

**Theory:** Wasm is faster than EVM

<!-- .element: class="fragment" data-fragment-index="1" -->

**Practice:** By the time Wasm has finished compiling, EVM executed a token swap literally dozens to hundreds of times

<!-- .element: class="fragment" data-fragment-index="2" -->

---

## Some more problems with Wasm

- Complexity of the Wasm spec
  - Tailored towards web2
  - Spec is open but our request were turned down in the past
- Compiler bombs:
  - Optimizing compilers are not executing in linear time
  - Which is bad because it opens a DOS attack vector
  - Metered compilation slows down work that is already expensive

---

# PolkaVM (PVM)

# Let's do better!

---

# PolkaVM

PolkaVM is based on RISC-V.

<!-- .element: class="fragment" data-fragment-index="1" -->

<img src="./img/pvm/RISC-V.svg" style="width: 100%">

<!-- .element: class="fragment" data-fragment-index="2" -->

---

# RISC-V

Wait a minute!

<!-- .element: class="fragment" data-fragment-index="1" -->

RISC-V isn't a platform agnostic bytecode.

<!-- .element: class="fragment" data-fragment-index="2" -->

But a real CPU?

<!-- .element: class="fragment" data-fragment-index="3" -->

---

## RISC-V

<img src="./img/pvm/risc-v-microcontroller.jpg">

---

## RISC-V

<img src="./img/pvm/risc-v-ledcube.png">

---

## RISC-V

Realizations:

- RISC-V is simple
- Practically a common denominator of widely used _real_ CPUs
  - x86_64
  - aarch64

---

## RISC-V as PAB solves many Wasm problems

- Determinism
  - We are not dictated by web2
  - Instead we can have our own requirements
  - => Solved simply because we can define it as requirement
- Compilation
  - It follows that RISC-V bytecode is (much!) simpler to compile
  - => Minimze the work done on-chain!

---

## RISC-V compilation

<img src="./img/pvm/on-off-chain-compilation-1.svg">

---

## RISC-V compilation

<img src="./img/pvm/on-off-chain-compilation-2.svg">

---

## RISC-V compilation vs. Wasm

Prompt: _Write me a fibonacci in RISC-V_

```
fibonacci:
    li t0, 1          # Load constant 1 into t0
    ble a0, t0, .base_case # If n <= 1, return n
    li t1, 0          # a = 0
    li t2, 1          # b = 1
.loop:
    add t3, t1, t2    # temp = a + b
    mv t1, t2         # a = b
    mv t2, t3         # b = temp
    addi a0, a0, -1   # n = n - 1
    bgt a0, t0, .loop # If n > 1, continue loop
    mv a0, t2         # Return b (Fibonacci(n))
    ret
.base_case:
    ret
```

---

<section>
  <h3>Intuitively: What's less work to compile to x86?</h3>
  <h3>Wasm or RISC-V?</h3>
  <table width="100%">
  <thead>
    <tr>
      <th>Wasm</th>
      <th>x86_64</th>
      <th>RISC-V</th>
    </tr>
  </thead>
    <tr>
      <td>
        <pre><code data-trim data-noescape>
        (loop $loop
          (if (i32.le_s ..)
            (then
              (return (local.get $n))
            )
          )
        ..
        )
        </code></pre>
      </td>
      <td>
        <pre><code data-trim data-noescape>
        fibonacci:
            cmp rdi, 1
            jle .base_case
            mov rax, 0
            mov rbx, 1
            mov rcx, rdi
        .loop:
            add rax, rbx
            xchg rax, rbx
            dec rcx
            jg .loop
            mov rax, rbx
            ret
        .base_case:
            mov rax, rdi
            ret
        </code></pre>
      </td>
      <td>
        <pre><code data-trim data-noescape>
        fibonacci:
            li t0, 1
            ble a0, t0, .base_case
            li t1, 0
            li t2, 1
        .loop:
            add t3, t1, t2
            mv t1, t2
            mv t2, t3
            addi a0, a0, -1
            bgt a0, t0, .loop
            mv a0, t2
            ret
        .base_case:
            ret
        </code></pre>
      </td>
    </tr>
  </table>
</section>

---

## The last puzzle piece

- PVM is based on the rv64**e**mac (_embedded_) ISA
  - The _embedded_ ISA is very similar to the standard ISA;
  - Reduces the general purpose register (GPA) count to 16
- Any computer will have 16 or more GPR (for the forseeable future)
  - x86_64 has 16 GPRs
  - aarch64 has 31 GPRs
- What this allows:
  - Instead of doing register allocation;
  - Compiling PVM bytecode is mostly a linear mapping of registers and instructions!

---

## This slide is important to understand

<pba-cols>
<pba-col center>

<img src="./img/pvm/pvm-compilation-meme.jpg">

<!-- .element: class="fragment" data-fragment-index="1" -->

</pba-col>
<pba-col center>

PVM bytecode is

- very close to machine code
- cheap to compile **on-chain**
- efficient to execute
- heavily optimized **off-chain**

</pba-col>
</pba-cols>

---

## PVM bytecode interpreter vs. JIT compiler

<img src="./img/pvm/on-off-chain-compilation-2.svg">

---

# PVM Benchmarks

<img src="./img/pvm/benchmarks-1.png" style="width: 100%">

---

# PVM Benchmarks

<img src="./img/pvm/benchmarks-2.png" style="width: 100%">

# EVM vs. PVM

- EVM is slower because it has to interpreted
- EVM isn't a general purpose VM
  - Pinky benchmarks not possible on EVM
  - EVM doesn't run contracts written other languages, e.g. Rust

---

# PVM beyond contracts

- Parachain Validation Function (PVF)
- Join-accumulate-machine (JAM)
  - https://github.com/gavofyork/graypaper
  - https://graypaper.fluffylabs.dev/

---

# PVM Demo

## Doom

Notes:

If we have enough time for it

---

# Summary

- PoklaVM solves blockchain specific pain points of Wasm
  - Inderministic execution
  - Compilation heavy-liftings moved off-chain
- Fast
- General purpose
