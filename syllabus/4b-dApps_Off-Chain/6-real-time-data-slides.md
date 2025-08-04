---
title: Dealing with real-time data
description: Concurrency models when building off-chain dApps
---

# Real-Time Data

---

## Why Concurrency is Hard

- We're used to think synchronously
- Combining synchronous code with async code is harder
- Things to look for
  - Race conditions
  - Data overwrites
  - Backpressure
- ~9% of bugs are due to concurrency
  - Hard to reproduce
  - Hard to debug

Notes:

https://arxiv.org/abs/2103.12447

---

## Why Real-Time Data is Hard

- Concurrency is everywhere
- More things to look for
  - Missed updates
  - Multiple sources of truth
  - Even more backpressure and resource limits

---

## Find the bug

```tsx
const Balance: FC<{ address: string }> = ({ address }) => {
  const [balance, setBalance] = useState(null);

  useEffect(() => {
    // Set to loading
    setBalance(null);
    // Load and display the balance
    loadBalance(address, result => setBalance(result));
  }, [address]);

  if (balance == null) {
    return <div>Loading…</div>;
  }

  return (
    <div>
      Balance of {address}: {balance}
    </div>
  );
};
```

Notes:

Notice how reproducing this bug is hell.

---

## Concepts

### Synchronous code

```ts
function calculateAverage(winnings: number) {
  const sum = winnings.reduce((a, b) => a + b, 0);
  return sum / winnings.length;
}
```

### Asynchronous code

```ts
function getAverage() {
  return fetch("/winnings").then(winnings => {
    return calculateAverage(winnings);
  });
}

// Or
async function getAverage() {
  const winnings = await fetch("/winnings");
  return calculateAverage(winnings);
}
```

---

## Concepts

<pba-cols style="font-size: 0.8em">
<pba-col>

### Concurrency

- Multiple tasks to execute simultaneously
- One single thread of execution
- Pause at specific points to work on another task
- No shared-memory races

</pba-col>
<pba-col>

### Parallelism

- Multiple threads execute tasks simultaneously
- Shared memory access
- Locks/mutexes
- Message passing

</pba-col>
</pba-cols>

---

## JavaScript Concurrency Model

---

### Single-Threaded Event Loop

- JS uses event loop
  - Macrotasks: I/O, `setTimeout`
  - Microtasks: `Promise.then`, `queueMicrotask`
- No multithreading by default
  - Available through WebWorkers

---

### Event Loop in Action

```ts
console.log("start");

setTimeout(() => console.log("timeout"), 0);

Promise.resolve().then(() => console.log("microtask"));

console.log("end");

// Logs: ?
```

```ts
// start
// end
// microtask
// timeout
```

<!-- .element: class="fragment" -->
