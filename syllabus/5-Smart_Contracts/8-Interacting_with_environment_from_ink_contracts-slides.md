---
title: Interacting with the execution environment from ink! smart contracts
---

<img rounded style="width: 600px;" src="./img/ink/ink-logo-with-squid-white.svg" />

# Interacting with the execution environment from ink! smart contracts

---

## What does the contract have access to?

<br/>

- message arguments
- its own storage data

<img style="margin-top: 50px;margin-bottom: 50px" width="600" src="./img/ink/ink-env-isolated.png" />

---

## What does the contract have access to?

<br/>

- message arguments
- its own storage data
- <font color="#c2ff33">host functions</font>

<img style="margin-top: 50px;margin-bottom: 50px" width="600" src="./img/ink/ink-env-not-isolated.png" />

---

## Blockchain onion - recap

<br/>

<img style="margin-top: 50px;margin-bottom: 50px" width="700" src="./img/ink/ink-env-onion-2.png" />

---

## Blockchain onion - recap

<br/>

<img style="margin-top: 50px;margin-bottom: 50px" width="700" src="./img/ink/blockchain-onion-3.svg" />

---

## Standard API

<br/>

- `caller()`
- `account_id()`
- `balance()`
- `block_number()`
- `emit_event(event: Event)`
- `transfer(dest: AccountId, value: Balance)`
- `hash_bytes(input: &[u8], output: &mut [u8])`
- `debug_message(msg: &str)`
- [_and many more_](https://docs.rs/ink_env/4.2.1/ink_env/index.html#functions)

---

## Standard API

```rust
impl MyContract {
  ...
  #[ink(message)]
  pub fn terminate(&mut self) -> Result<()> {
      let caller = self.env().caller();
      self.env().terminate_contract(caller)
  }
  ...
}
```

---

## Interacting with the state transition function

<br/>

<div style="display: flex; justify-content: center; align-items: center;">
    <div style="flex: 1; text-align: center;">
        <div style="text-align: center"> <center><h2><pre> User API </pre></h2></center> </div>
        <ul>
            <li>token transfer</li>
            <li>staking</li>
            <li>voting</li>
            <li>contract call</li>
            <li>...</li>
        </ul>
    </div>
    <div style="flex: 1;">
        <div style="text-align: center"> <center><h2><pre> Contract API </pre></h2></center> </div>
        <ul>
            <li>advanced cryptography</li>
            <li>bypassing standard restrictions</li>
            <li>outsourcing computation</li>
            <li>...</li>
        </ul>
    </div>
</div>

---

## Interacting with the state transition function

<br/>

<div style="display: flex; justify-content: center; align-items: center;">
    <div style="flex: 1; text-align: center;">
        <div style="text-align: center"> <center><h2><pre> User API </pre></h2></center> </div>
        <div style="text-align: center"> <center><h2><pre> (usually for humans) </pre></h2></center> </div>
        <ul>
            <li>token transfer</li>
            <li>staking</li>
            <li>voting</li>
            <li>contract call</li>
            <li>...</li>
            <br/>
            <font color="#c2ff33">runtime call</font>
        </ul>
    </div>
    <div style="flex: 1;">
        <div style="text-align: center"> <center><h2><pre> Contract API </pre></h2></center> </div>
        <div style="text-align: center"> <center><h2><pre> (only for contracts) </pre></h2></center> </div>
        <ul>
            <li>advanced cryptography</li>
            <li>bypassing standard restrictions</li>
            <li>outsourcing computation</li>
            <li>...</li>
            <br/>
            <font color="#c2ff33">chain extension</font>
        </ul>
    </div>
</div>

---

## Interacting with the state transition function

<br/>

<div style="display: flex; justify-content: center; align-items: center;">
    <div>
        <img style="margin-top: 50px;margin-bottom: 50px" width="70" src="./img/ink/ink-env-fiona-call.png" />
    </div>
    <div style="flex: 1; text-align: center;">
        <div style="text-align: center"> <center><h2><pre> User API </pre></h2></center> </div>
        <div style="text-align: center"> <center><h2><pre> (usually for humans) </pre></h2></center> </div>
        <ul>
            <li>token transfer</li>
            <li>staking</li>
            <li>voting</li>
            <li>contract call</li>
            <li>...</li>
            <br/>
            <font color="#c2ff33">runtime call</font>
        </ul>
    </div>
    <div>
        <img style="margin-top: 50px;margin-bottom: 50px" width="120" src="./img/ink/ink-env-fiona-extension.png" />
    </div>
    <div style="flex: 1;">
        <div style="text-align: center"> <center><h2><pre> Contract API </pre></h2></center> </div>
        <div style="text-align: center"> <center><h2><pre> (only for contracts) </pre></h2></center> </div>
        <ul>
            <li>advanced cryptography</li>
            <li>bypassing standard restrictions</li>
            <li>outsourcing computation</li>
            <li>...</li>
            <br/>
            <font color="#c2ff33">chain extension</font>
        </ul>
    </div>
</div>

---

## Interacting with the state transition function

<table style="display: flex; justify-content: center; align-items: center">
    <tr>
        <td style="width:400px; border: 2px solid white"></td>
        <td style="width:400px; border: 2px solid white">
            <figure>
                <img style="margin-top: 50px;margin-bottom: -20px" width="70" src="./img/ink/ink-env-fiona-call.png" /> 
                <figcaption>User API</figcaption>
            </figure>
        </td>
        <td style="width:400px; border: 2px solid white">
            <figure>
                <img style="margin-top: 50px;margin-bottom: -20px" width="100" src="./img/ink/ink-env-fiona-extension.png" /> 
                <figcaption>Contract API</figcaption>
            </figure>
        </td>
    </tr>
    <tr>
        <td style="width:400px; border: 2px solid white">
            <figure>
                <img style="margin-top: 50px;margin-bottom: -20px" width="120" src="./img/ink/ink-env-farquad.png" /> 
                <figcaption>Standard blockchain<br/>user (human)</figcaption>
            </figure>
        </td>
        <td style="width:400px; border: 2px solid white"></td>
        <td style="width:400px; border: 2px solid white"></td>
    </tr>
    <tr>
        <td style="width:400px; border: 2px solid white">
            <figure>
                <img style="margin-top: 50px;margin-bottom: -20px" width="120" src="./img/ink/ink-env-shrek.png" /> 
                <figcaption>Smart contract</figcaption>
            </figure>
        </td>
        <td style="width:400px; border: 2px solid white"></td>
        <td style="width:400px; border: 2px solid white"></td>
    </tr>

</table>

---

## Interacting with the state transition function

<table style="display: flex; justify-content: center; align-items: center">
    <tr>
        <td style="width:400px; border: 2px solid white"></td>
        <td style="width:400px; border: 2px solid white">
            <figure>
                <img style="margin-top: 50px;margin-bottom: -20px" width="70" src="./img/ink/ink-env-fiona-call.png" /> 
                <figcaption>User API</figcaption>
            </figure>
        </td>
        <td style="width:400px; border: 2px solid white">
            <figure>
                <img style="margin-top: 50px;margin-bottom: -20px" width="100" src="./img/ink/ink-env-fiona-extension.png" /> 
                <figcaption>Contract API</figcaption>
            </figure>
        </td>
    </tr>
    <tr>
        <td style="width:400px; border: 2px solid white">
            <figure>
                <img style="margin-top: 50px;margin-bottom: -20px" width="120" src="./img/ink/ink-env-farquad.png" /> 
                <figcaption>Standard blockchain<br/>user (human)</figcaption>
            </figure>
        </td>
        <td style="width:400px; border: 2px solid white; text-align: center; vertical-align: middle"><font color="#c2ff33">YES</font></td>
        <td style="width:400px; border: 2px solid white; text-align: center; vertical-align: middle"><font color="#e6007a">NO</font></td>
    </tr>
    <tr>
        <td style="width:400px; border: 2px solid white">
            <figure>
                <img style="margin-top: 50px;margin-bottom: -20px" width="120" src="./img/ink/ink-env-shrek.png" /> 
                <figcaption>Smart contract</figcaption>
            </figure>
        </td>
        <td style="width:400px; border: 2px solid white; text-align: center; vertical-align: middle"><font color="#c2ff33">YES</font></td>
        <td style="width:400px; border: 2px solid white; text-align: center; vertical-align: middle"><font color="#c2ff33">YES</font></td>
    </tr>

</table>

---

## Interacting with the state transition function

<table style="display: flex; justify-content: center; align-items: center">
    <tr>
        <td style="width:400px; border: 2px solid white"></td>
        <td style="width:400px; border: 2px solid white">
            <figure>
                <img style="margin-top: 50px;margin-bottom: -20px" width="70" src="./img/ink/ink-env-fiona-call.png" /> 
                <figcaption>User API</figcaption>
            </figure>
        </td>
        <td style="width:400px; border: 2px solid white">
            <figure>
                <img style="margin-top: 50px;margin-bottom: -20px" width="100" src="./img/ink/ink-env-fiona-extension.png" /> 
                <figcaption>Contract API</figcaption>
            </figure>
        </td>
    </tr>
    <tr>
        <td style="width:400px; border: 2px solid white">
            <figure>
                <img style="margin-top: 50px;margin-bottom: -20px" width="120" src="./img/ink/ink-env-farquad.png" /> 
                <figcaption>Standard blockchain<br/>user (human)</figcaption>
            </figure>
        </td>
        <td style="width:400px; border: 2px solid white; text-align: center"></td>
        <td style="width:400px; border: 2px solid white; text-align: center"></td>
    </tr>
    <tr>
        <td style="width:400px; border: 2px solid white">
            <figure>
                <img style="margin-top: 50px;margin-bottom: -20px" width="120" src="./img/ink/ink-env-shrek.png" /> 
                <figcaption>Smart contract</figcaption>
            </figure>
        </td>
        <td style="width:400px; border: 2px solid white; text-align: center; vertical-align: middle"><font color="#c2ff33">runtime call</font></td>
        <td style="width:400px; border: 2px solid white; text-align: center; vertical-align: middle"><font color="#c2ff33">chain extension</font></td>
    </tr>

</table>

---

## Calling runtime

<br/>

```rust [7-10]
#[ink(message)]
pub fn transfer_through_runtime(
    &mut self,
    receiver: AccountId,
    value: Balance,
) -> Result<(), RuntimeError> {
    let call_object = RuntimeCall::Balances(BalancesCall::Transfer {
        receiver,
        value,
    });

    self.env().call_runtime(&call_object)
}
```

---

## Calling runtime

<br/>

```rust [12]
#[ink(message)]
pub fn transfer_through_runtime(
    &mut self,
    receiver: AccountId,
    value: Balance,
) -> Result<(), RuntimeError> {
    let call_object = RuntimeCall::Balances(BalancesCall::Transfer {
        receiver,
        value,
    });

    self.env().call_runtime(&call_object)
}
```

---

## Runtime call - important notice

<br/>

The runtime must be explicitly configured (by the runtime maintainers) to allow particular call type.

---

## Runtime call - pros and cons

<br/>

<div style="display: flex; justify-content: center; align-items: center;">
    <div style="flex: 1; text-align: center;">
        <div style="text-align: center"> <center><h2><pre> Pros </pre></h2></center> </div>
        <ul>
            <li>The code is already there (with its own security, benchmarking, error handling, etc.).</li>
            <li>Contract can act on behalf of a user.</li>
        </ul>
    </div>
    <div style="flex: 1;">
        <div style="text-align: center"> <center><h2><pre> Cons </pre></h2></center> </div>
        <ul>
            <li>If the call API is changed, contract is broken.</li>
            <li>Runtime must be configured to allow the call.</li>
            <li>Cannot return any value.</li>
        </ul>
    </div>
</div>

---

## Chain extensions

<br/>

Chain extension is a way to extend the runtime with custom functionalities _dedicated to contracts_.

---

## Chain extensions

<br/>

**ink! side:**

- provide `ChainExtension` trait
- include extension in the `Environment` trait instantiation

<br/>

**runtime side:**

- handling extension calls
- extension logic itself

---

## Provide `ChainExtension` trait

```rust [1-7]
#[ink::chain_extension]
pub trait OutsourceHeavyCrypto {
  type ErrorCode = OutsourcingErr;

  #[ink(extension = 41)]
  fn outsource(input: Vec<u8>) -> [u8; 32];
}

pub enum OutsourcingErr {
  IncorrectData,
}

impl ink::env::chain_extension::FromStatusCode for OutsourcingErr {
  fn from_status_code(status_code: u32) -> Result<(), Self> {
    match status_code {
      0 => Ok(()),
      1 => Err(Self::IncorrectData),
      _ => panic!("encountered unknown status code"),
    }
  }
}
```

---

## Provide `ChainExtension` trait

```rust [9-21]
#[ink::chain_extension]
pub trait OutsourceHeavyCrypto {
  type ErrorCode = OutsourcingErr;

  #[ink(extension = 41)]
  fn outsource(input: Vec<u8>) -> [u8; 32];
}

pub enum OutsourcingErr {
  IncorrectData,
}

impl ink::env::chain_extension::FromStatusCode for OutsourcingErr {
  fn from_status_code(status_code: u32) -> Result<(), Self> {
    match status_code {
      0 => Ok(()),
      1 => Err(Self::IncorrectData),
      _ => panic!("encountered unknown status code"),
    }
  }
}
```

---

## Include extension in the `Environment` trait instantiation

<br/>

```rust
pub enum EnvironmentWithOutsourcing {}
impl Environment for EnvironmentWithOutsourcing {
    ... // use defaults from `DefaultEnvironment`
    type ChainExtension = OutsourceHeavyCrypto;
}

#[ink::contract(env = crate::EnvironmentWithOutsourcing)]
mod my_contract {
  ...
}
```

---

## Include extension in the `Environment` trait instantiation

<br/>

```rust
#[ink::contract(env = crate::EnvironmentWithOutsourcing)]
mod my_contract {
  fn process_data(&mut self, input: Vec<u8>) -> Result<(), OutsourcingErr> {
    self.env().extension().outsource(subject)
  }
}
```

---

## Handling extension calls

<br/>

```rust [5-11]
pub struct HeavyCryptoOutsourcingExtension;

impl ChainExtension<Runtime> for HeavyCryptoOutsourcingExtension {
  fn call<E: Ext>(&mut self, env: Env) -> Result<RetVal, DispatchError> {
    match env.func_id() {
      41 => internal_logic(),
      _ => {
        error!("Called an unregistered `func_id`: {func_id}");
        return Err(DispatchError::Other("Unimplemented func_id"))
      }
    }
    Ok(RetVal::Converging(0))
}
```

---

## Chain extension - pros and cons

<br/>

<div style="display: flex; justify-content: center; align-items: center;">
    <div style="flex: 1; text-align: center;">
        <div style="text-align: center"> <center><h2><pre> Pros </pre></h2></center> </div>
        <ul>
            <li>Arbitrary logic dedicated to contracts.</li>
            <li>Customized weighting, limitations, etc.</li>
            <li>Can wrap (e.g. for compatibility) runtime calls.</li>
            <li>Can return value.</li>
        </ul>
    </div>
    <div style="flex: 1;">
        <div style="text-align: center"> <center><h2><pre> Cons </pre></h2></center> </div>
        <ul>
            <li>Requires own maintenance, security audit, benchmarking.</li>
            <li>Very high complexity</li>
        </ul>
    </div>
</div>

---

## Chain extension: important note

As with the call runtime, runtime maintainers must include particular extension in the runtime.
And it’s also usually them, who actually creates ink! side of the extension.

---

## Chain extension: reaching even further

<img style="margin-top: 100px;margin-bottom: 50px" width="800" src="./img/ink/chain-extension-reach.svg" />

---

## Third way?

There are plans to integrate contracts with XCM…
