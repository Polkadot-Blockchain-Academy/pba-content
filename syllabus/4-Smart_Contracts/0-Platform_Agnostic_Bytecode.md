---
title: Platform Agnostic Bytecode
description: What are PABs and why they exist?
duration: 1 hour
---

# Platform Agnostic Bytecode

---

## Definition

A PAB is a bytecode that follows two main principles: 

- Turing Completness, as a standard bytecode would respect 

<!-- .element: class="fragment" data-fragment-index="1" -->

- Support for tooling that makes it executable on every machine 

<!-- .element: class="fragment" data-fragment-index="2" -->

---

## Bytecodes

<pba-cols>
<pba-col left>

<pba-flex center>

###### High Level Languages

<img style="height: 15vh" src="img/pab/rust_logo.png" alt="Rust logo" />

<img style="height: 15vh" src="img/pab/c_logo.png" alt="C logo" />

<img style="height: 15vh" src="img/pab/c++_logo.png" alt="C++ logo" />

...

</pba-flex>

</pba-col>
<!-- .element: class="fragment" data-fragment-index="1" -->
<pba-col center>

<pba-flex center>

###### PABs

<img style="height: 15vh" src="img/pab/jvm_logo.png" alt="jvm logo" />
<img style="height: 15vh" src="img/pab/wasm_logo.png" alt="wasm logo" />
<img style="height: 15vh" src="img/pab/eth_logo.png" alt="evm logo" />

...
</pba-flex>

</pba-col>
<!-- .element: class="fragment" data-fragment-index="2" -->
<pba-col right>

<pba-flex center>

###### Architecture's bytecode

<img style="height: 10vh" src="img/pab/intel_logo.png" alt="intel logo" />

<img style="width: 15vh" src="img/pab/arm_logo.png" alt="arm logo" />

<img style="width: 20vh" src="img/pab/risc-v_logo.png" alt="RISC-V logo" />?!

</pba-flex>

</pba-col>
<!-- .element: class="fragment" data-fragment-index="3" -->
</pba-cols>

---

#### What a PAB allows is:

<pba-flex center>

- Portability
<!-- .element: class="fragment" data-fragment-index="1" -->
    - Block Hardware Centralization
<!-- .element: class="fragment" data-fragment-index="3" -->
- Determinism
<!-- .element: class="fragment" data-fragment-index="2" -->
    - Make consensus possible
<!-- .element: class="fragment" data-fragment-index="4" -->

</pba-flex>

---v


##### That's why PABs are so important

---

## PABs Rating
Features that a PAB should follow:

- Hardware Independence
<!-- .element: class="fragment" data-fragment-index="1" -->
- Efficency
<!-- .element: class="fragment" data-fragment-index="2" -->
- Tool Simlicity
<!-- .element: class="fragment" data-fragment-index="3" -->
- Support as Compilation Target
<!-- .element: class="fragment" data-fragment-index="4" -->
- Sandboxing
<!-- .element: class="fragment" data-fragment-index="5" -->

---v

### Sanboxing?

A sandbox environment is where potentially unsafe software code can be executed without affecting the security of the machine in which is executed.
<!-- .element: class="fragment" data-fragment-index="1" -->

A sandboxed environment must be created by the executor of the PAB.
<!-- .element: class="fragment" data-fragment-index="2" -->

A SmartContract is *Arbitrary Code* that should be executed by multiple nodes -> we don't want SmartContracts capable of destroying the nodes on which they are executed 
<!-- .element: class="fragment" data-fragment-index="3" -->

---

<!-- .slide: data-background-color="#4A2439" -->

<img rounded style="width: 300px" src="img/ink/question-mark.svg" />

---

<pba-cols>
<pba-col center>

# WebAssembly 
<!-- .element: class="fragment" data-fragment-index="1" -->

</pba-col>
<pba-col center>

<img style="height: 30vh" src="https://upload.wikimedia.org/wikipedia/commons/thumb/1/1f/WebAssembly_Logo.svg/1200px-WebAssembly_Logo.svg.png" alt="wasm logo" />
</pba-col>
</pba-cols>

---

## Wasm's keypoins

<pba-flex center>

- binary instruction format for a stack-based virtual machine
<!-- .element: class="fragment" data-fragment-index="1" -->
- Supported as compilation target by many languages
<!-- .element: class="fragment" data-fragment-index="2" -->
  - Rust, C, C++ and many others
<!-- .element: class="fragment" data-fragment-index="3" -->
- Fast (with near-native performance)
<!-- .element: class="fragment" data-fragment-index="4" -->
- Safe (executed in a sandoxed anvironment)
<!-- .element: class="fragment" data-fragment-index="5" -->
- Hardware-independent
<!-- .element: class="fragment" data-fragment-index="6" -->
- Open
<!-- .element: class="fragment" data-fragment-index="7" -->

</pba-flex>

---
## Stack-Based Virtual Machine Example

<pba-cols>
<pba-col center>

``` wasm 
(module
  (import "console" "log" (func $log (param i32)))
  (func $main
    ;; load `10` and `3` onto the stack
    i32.const 10
    i32.const 3

    i32.add ;; add up both numbers
    call $log ;; log the result
  )
  (start $main)
)
```
<!-- .element: class="fragment" data-fragment-index="0" -->

</pba-col>
<pba-col center>

<img src="img/pab/stack_5.png" height="200vh">
<!-- .element: class="fragment" data-fragment-index="1" -->

<img src="img/pab/stack_5_10.png" height="200vh">
<!-- .element: class="fragment" data-fragment-index="2" -->

<img src="img/pab/stack_15.png" height="200vh">
<!-- .element: class="fragment" data-fragment-index="3" -->

</pba-col>
</pba-cols>

---

## Wasm seems to be a perfect PAB, but

- How works the communication with the environemt?
<!-- .element: class="fragment" data-fragment-index="1" -->
- How is it sandboxed?
<!-- .element: class="fragment" data-fragment-index="2" -->
- How can we compile to WASM? (maybe not needed)
<!-- .element: class="fragment" data-fragment-index="3" -->
- How is it executed on-chain?
<!-- .element: class="fragment" data-fragment-index="4" -->

Notes:

To demonstrate why Wasm is (almost) a perfect PAB and why Polkadot decide to use will be explained how:

---

## Communication with the Environemnt

How can the wasm blob communicate with the environment in which is executed?
<!-- .element: class="fragment" data-fragment-index="0" -->

---v

### Problem

**Wasm has no ambient access to the computing environment in which code is executed**
<!-- .element: class="fragment" data-fragment-index="0" -->

### Solution
<!-- .element: class="fragment" data-fragment-index="1" -->

- Every interaction can be done by only a set of functions provided by the embedder and imported in wasm
<!-- .element: class="fragment" data-fragment-index="2" -->
  - Called: **Host Functions**
<!-- .element: class="fragment" data-fragment-index="2" -->
- The embedder is able to call only the functions provided by the wasm blob
<!-- .element: class="fragment" data-fragment-index="3" -->
  - Called: **Runtime API**, later in the course you will understand why they are called by that 
<!-- .element: class="fragment" data-fragment-index="3" -->

How data are passed between two worlds is managed by something called FFI 
<!-- .element: class="fragment" data-fragment-index="5" -->

---

## Sanboxing

Wasm has only access to memory provided by the embedder, this memory is also called **Liner Memory**.
<!-- .element: class="fragment" data-fragment-index="0" -->

</br>

- This area will be used as a frontier for data sharing
- To make everything secure the Embedder is doing incredibly convoluted things (resources linked)

<!-- .element: class="fragment" data-fragment-index="1" -->

---

## How WASM is executed on-chain

There are multiple ways to execute wasm, the ones used in polkadot are:

- Ahead Of Time Compilation (not sure if substrate-based chain uses JIT or AOT)
- Interpretation

<!-- .element: class="fragment" data-fragment-index="0" -->

But also other type of execution exists, for example:
<!-- .element: class="fragment" data-fragment-index="1" -->
- Just in Time Compilation
- Single Pass Compilation
- Stream Compilation

<!-- .element: class="fragment" data-fragment-index="1" -->

---v

### Wasmtime

- It is a stand alone wasm environment
- Used in substrate to execute the blockchain logic
- It compiles once the wasm blob 
- It executes the compiled wasm blob in sanboxed environment while keeping everything extremely secure

---v
### Wasmi

- It also is a wasm environment
- It interpret the wasm blob and execute it securly 
- The Interpreter itself can be built in wasm itself

Due to it's characteristics it is mainly used to execute SmartContracts on chain
<!-- .element: class="fragment" data-fragment-index="1" -->


---

# Alternatives

- EVM
- CosmWasm
- Solana eBPF
- RISC-V ?!

---

