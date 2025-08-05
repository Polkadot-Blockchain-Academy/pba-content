---
title: Dealing with real-time data
description: Concurrency models when building off-chain dApps
---

# Real-Time Data

---v

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

---v

## Why Real-Time Data is Hard

- Concurrency is everywhere
- More things to look for
  - Missed updates
  - Multiple sources of truth
  - Even more backpressure and resource limits

---v

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

---v

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

---v

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

---v

### Single-Threaded Event Loop

- JS uses event loop
  - Macrotasks: I/O, `setTimeout`
  - Microtasks: `Promise.then`, `queueMicrotask`
- No multithreading by default
  - Available through WebWorkers

---v

### Event Loop in Action

```ts
console.log("start");

setTimeout(() => console.log("timeout"), 0);

Promise.resolve().then(() => console.log("promise.then"));

console.log("end");

// Logs: ?
```

```ts
// start
// end
// promise.then
// timeout
```

<!-- .element: class="fragment" -->

---v

### Event Loop in Action

```ts
const promiseSomewhereElse = Promise.resolve(123);

async function doStuff() {
  console.log("doStuff");

  const value = await promiseSomewhereElse;

  console.log("result", value);
}

setTimeout(() => console.log("timeout"), 0);
Promise.resolve().then(() => console.log("promise.then"));
doStuff();

// Logs: ?
```

```ts
// doStuff
// promise.then
// result 123
// timeout
```

<!-- .element: class="fragment" -->

---v

### Re-entrant calls

```ts
function createCounter(onFirstIncrement: () => void) {
  let count = 0;
  return () => {
    count++;
    if (count === 1) {
      onFirstIncrement();
    }
    console.log("incremented to " + count);
  };
}

const increment = createCounter(() => {
  // Trigger another increment when the first one happens;
  increment();
});
increment();
```

Everything is synchronous, yet it can also have unexpected effects.

---v

### Key take-_awaits_

- Synchronous blocks are not interrupted
  - Watch out for re-entrant calls!
- JS uses the event loop to schedule the tasks
  - Macrotasks: I/O, `setTimeout`
  - Microtasks: Promises
- The event loop runs all microtasks before starting a new macrotask

---

## Let's dive Async

---v

### Pull vs Push

<pba-cols style="font-size: 0.8em">
<pba-col>

<!-- prettier-ignore -->
```ts
let lastValue = null;
while (keepWatching) {
  const value = await typedApi
    .query.System.Account
    .getValue(ACCOUNT_ID);

  if (value !== lastValue) {
    console.log("new value", value);
  }
  lastValue = value;

  await waitMs(1000);
}
```

</pba-col>
<pba-col>

<!-- prettier-ignore -->
```ts
typedApi
  .query.System.Account
  .watchValue(ACCOUNT_ID)
  .subscribe(value => {
    console.log("new value", value);
  })
```

</pba-col>
</pba-cols>

---v

<pba-cols style="font-size: 0.8em">
<pba-col>

### Pull

Consumer decides when to get the value

</pba-col>
<pba-col>

### Push

Producer notifies of new changes

</pba-col>
</pba-cols>

---v

### JSON-RPC Spec

- ChainHead Events: <span class="fragment">Push-based</span>
- Operations: <span class="fragment">Push/pull?</span>
  - Low-level: Push <!-- .element: class="fragment" --->
  - High-level: Pull <!-- .element: class="fragment" --->
  - Higher-level: Push 🤯 <!-- .element: class="fragment" --->

Notes:

Events (new block, finalized, etc) are push-based, the node notifies us when a new block is produced
Operations depend on the level we're looking at.

---v

### Working Async

- Push is asynchronous
- Pull can be sync or async
- Ancient JS (2014-) used callbacks for async code

```ts
// Ancient JS Pull
api.query.System.Account.getValue(ACCOUNT_ID, (error, result) => {
  if (error) {
    return console.error("oh no!");
  }
  console.log("Result", result);
});

// Ancient JS Push
api.query.System.Account.watchValue(
  ACCOUNT_ID,
  value => {
    console.log("Value", value);
  },
  error => console.error("oh no!")
);
```

Notes:

In our context, pull is async. If you need the storage, you have to make a request to the node to pull it out for you.

---v

### Enter Promises 🌈

- Technically still using callbacks.
- Common interface, allows composability.
- Removes "callback hell"
- 2017+ enhanced language: async/await
- Works great for pull operations: fetch

---v

### And for push?

😞 <!-- .element: class="fragment" --->

<div class="fragment">

Async generators?

```ts
// async generator API
const account = api.query.System.Account.watchValue(ACCOUNT_ID);

for await (const value of account) {
  console.log(value);
}
```

But that's pull!

</div>

---v

### Enter Observables 🌈

- Promise but for multiple values
- Common interface, composable
- Removes "callback hell"
- Convertible to promises
- TC-39 Stage 1 <span style="color: darkgray" class="fragment">(big copium)</span>
- Meanwhile rxjs <!-- .element: class="fragment" -->

---v

### Observables 101

```ts
import { Observable } from "rxjs";

// Emit one value every second up to 10
const observable$ = new Observable<number>(subscriber => {
  let v = 0;

  const token = setInterval(() => {
    const valueToEmit = v;
    v++;
    subscriber.next(valueToEmit);
    if (valueToEmit === 10) {
      subscriber.complete();
    }
  }, 1000);

  return () => clearInterval(token);
});
```

---v

### Observables 101

```ts
observable$.subscribe(value => {
  console.log(value);
});

observable$.subscribe({
  next: value => console.log(value),
  error: error => console.error(error),
  complete: () => console.log("completed"),
});
```

Notes:

Showcase / demo how observables are cold by default

Notice how the cold behaviour also helps avoid re-entrant subscriptions by design. They become relevant only when creating shared observables.

---v

### Composing Observables

Operator: `(source: Observable<T>) => Observable<R>`

```ts
const map =
  <T, R>(mapFn: (value: T) => R) =>
  (source: Observable<T>) =>
    new Observable<R>(subscriber => {
      const subscription = source.subscribe({
        next: v => subscriber.next(mapFn(v)),
        error: e => subscriber.error(e),
        complete: () => subscriber.complete(),
      });

      return subscription;
    });

const multipliedBy2$ = observable$.pipe(map(v => v * 2));
// Same as map(v => v * 2)(multipliedBy2$)
```

---v

### Composing Observables

Pipe-ing

```ts
import { interval, map, take } from "rxjs";

// Create an observable of the first 10 even numbers, one second at a time.
const even$ = interval(1000).pipe(
  map(v => v * 2),
  take(10)
);
```

---v

### Combining Observables

- `combineLatest`, `merge`, `switchMap`, etc.
- Endless list: https://rxjs.dev/guide/operators#creation-operators-1
- Good resource: https://www.learnrxjs.io/learn-rxjs/operators

---v

### Combining Observables

switchMap

```ts
const switchMap =
  <T, R>(mapFn: (value: T) => Observable<R>) =>
  (source: Observable<T>) =>
    new Observable<R>(subscriber => {
      // TODO

      const subscription = source.subscribe({
        next: v => {
          const innerObservable = mapFn(v);

          // TODO
        },
        error: e => subscriber.error(e),
        complete: () => subscriber.complete(),
      });

      return () => {
        // TODO
      };
    });

const bounty$ = selectedBountyId$.pipe(
  switchMap(id => {
    return from(typedApi.query.Bounties.Bounty.getValue(id));
  })
);
```

---v

### Observable ↔ Promise

```ts
import { firstValueFrom, lastValueFrom, from, defer } from "rxjs";
const firstValue = await firstValueFrom(observable$);
const lastValue = await lastValueFrom(observable$);

const observable$ = from(fetch("…"));
const observable$ = defer(() => fetch("…"));
```

Notes:

It's important to keep in mind what these functions do: both firstValueFrom and lastValueFrom subscribe to the observable, and then unsubscribe.

Difference between from and defer.

---v

### Polkadot Chains

- Pull
  - Constants (metadata)
  - Runtime APIs
  - Storage query
- Push
  - Blocks
  - Storage watch
  - Transactions

Notes:

Pull operations: Easier to offer promises
Push: Observables all the way.

- Blocks: finalized$

Why transactions are "push"?

---v

### Combining streams

Exercise: Find the referenda where a specific account voted in the same direction as the current outcome.

- Account: 1jbZxCFeNMRgVRfggkknf8sTWzrVKbzLvRuLWvSyg9bByRG
- Track: 33

Hints:

- `query.ConvictionVoting.VotingFor.watchValue(account, track)`
- `query.Referenda.ReferendumInfoFor.getValues([number][])`

Notes:

- Don't count delegations.
- Don't count split votes or abstains.

TODO Maybe find a better example. This one is interesting, but it's a shame it would need to do `watchValue[]` and that just adds boilerplate. Also, not a huge fan of the nesting and the hacks around conviction voting. Plus having to share the account which is just a random string…

```ts
const getDirectVotes = (voting: ConvictionVotingVoteVoting) => {
  if (voting.type === "Delegating") return [];

  return voting.value.votes
    .map(([id, vote]) => {
      if ("vote" in vote.value) {
        const direction = vote.value.vote & 0x80 ? "aye" : "nay";
        return { id, direction };
      }
      return null;
    })
    .filter(v => v !== null);
};

dotApi.query.ConvictionVoting.VotingFor.watchValue("1jbZxCFeNMRgVRfggkknf8sTWzrVKbzLvRuLWvSyg9bByRG", 33)
  .pipe(
    map(getDirectVotes),
    switchMap(async voting => {
      const referenda = await dotApi.query.Referenda.ReferendumInfoFor.getValues(voting.map(v => [v.id]));

      return referenda
        .filter(v => v != null)
        .filter((referendum, i) => {
          const { direction } = voting[i];
          if (referendum.type !== "Ongoing") {
            return (direction === "aye" && referendum.type === "Approved") || direction === "nay";
          }
          const referendumDirection = referendum.value.tally.ayes > referendum.value.tally.nays ? "aye" : "nay";
          return direction === referendumDirection;
        })
        .map((v, i) => ({
          ...v,
          id: voting[i].id,
        }));
    })
  )
  .subscribe(r => {
    console.log(r);
  });
```

TODO more non-papi exercises

---v

### Common Pain Points

- Repeated computations
  - With `combineLatest`
- Value-not-yet-there
  - With `withLatestFrom`
- Backpressure
  - Handling fast producers
