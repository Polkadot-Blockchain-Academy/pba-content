# Subkey Signature and KDKD Demo

All the subkey examples from the first draft have been translated into Rust in a Jupyter notebook to run in the lecture. Just in case there are any mishaps, here are all the subkey examples.

## Key Generation

```
> subkey generate
Secret phrase:       thank liberty fame metal illegal project behind join armed afraid welcome act
  Secret seed:       0xa959ae8546c355d25e7dacdffac94d6f5d2ee8c28a0b5b0511a60f0d2aa5c595
  Public key (hex):  0x7498c845882ffa046a994d3f4d0e6422fbc6ff85aa61f294ab293158e7e6381d
```

## Sign

```
> echo -n 'Hello Polkadot Blockchain Academy, Cambridge 2022' | subkey sign \
    --suri 'master ostrich insect boost sword cigar balance agent crater assist cheese play'
58dd5fafab68b27bf7019f10610438be7bab34309d418ce8ceff4c12c262a0598d98c9c4c412c57dff78cb7bd1fd4dfd0445ad8c3aa1192893db815e21f92b84
```

## Verify

```
> echo -n 'Hello Polkadot Blockchain Academy, Cambridge 2022' | subkey verify \
    '0x58dd5fafab68b27bf7019f10610438be7bab34309d418ce8ceff4c12c262a0598d98c9c4c412c57dff78cb7bd1fd4dfd0445ad8c3aa1192893db815e21f92b84' \
    '0x846ef3e2cdb5afc57c718762b1bdd761a3e85e8b4bc37c755d663b5c0e805b39'
Signature verifies correctly.
```

## Tamper with the Message

```
> echo -n 'Hello Polkadot Blockchain Academy, Cambridge 2021' | subkey verify \
	'0x58dd5fafab68b27bf7019f10610438be7bab34309d418ce8ceff4c12c262a0598d98c9c4c412c57dff78cb7bd1fd4dfd0445ad8c3aa1192893db815e21f92b84' \
    '0x846ef3e2cdb5afc57c718762b1bdd761a3e85e8b4bc37c755d663b5c0e805b39'
Error: SignatureInvalid
```

## Hard Derivation

```
> subkey inspect 'master ostrich insect boost sword cigar balance agent crater assist cheese play//polkadot'
Secret Key URI `master ostrich insect boost sword cigar balance agent crater assist cheese play//polkadot` is account:
  Secret seed:       0xae84d8131fc9639013c16927a9c97f0fb25f1ec110ec239705a84fbe51694d12
  Public key (hex):  0xfa8e9bae4f80275a1bf6a0b582461d949bbd64a06f82c817d236ba1f4193b502
  Account ID:        0xfa8e9bae4f80275a1bf6a0b582461d949bbd64a06f82c817d236ba1f4193b502
  SS58 Address:      5HjEC5U39iEJ71TDHLS7Tmzjsb412jNRkk4QPSz5DX18UQBu

> subkey inspect 'master ostrich insect boost sword cigar balance agent crater assist cheese play//kusama'
Secret Key URI `master ostrich insect boost sword cigar balance agent crater assist cheese play//kusama` is account:
  Secret seed:       0xf800549f4f3f0910e05476e30647411ef380642a4794b0c25efcaa1e60c97900
  Public key (hex):  0x12211e2cba7a01b5b1c086e7e40e56578843127109431848e129942555113957
  Account ID:        0x12211e2cba7a01b5b1c086e7e40e56578843127109431848e129942555113957
  SS58 Address:      5CUUZGVoWGY9LeFAJSBpDBVrZetTGiHfCMF4hsWK5JxMbK2Q
```

## Soft Derivation from Secret

```
> subkey inspect 'master ostrich insect boost sword cigar balance agent crater assist cheese play//polkadot/0'
Secret Key URI `master ostrich insect boost sword cigar balance agent crater assist cheese play//polkadot/0` is account:
  Secret seed:       n/a
  Public key (hex):  0xa68af0a0883030efb2d4eb7e7adb9a8de684508de90dc4b4a4f84232ff83ff3a
  SS58 Address:      5Fq55aj4dfriNvF8wmVoCJo5NiTx8xpN4XkpDEaZQKTgP5mc

> subkey inspect 'master ostrich insect boost sword cigar balance agent crater assist cheese play//polkadot/1'
Secret Key URI `master ostrich insect boost sword cigar balance agent crater assist cheese play//polkadot/1` is account:
  Secret seed:       n/a
  Public key (hex):  0x1e901acf62f90c32e10518063b30600a1bdf010d399ce6fb06350042b1b6af3c
  SS58 Address:      5Ckn6rXjTmsYABZSLcHQLEyfzegscFfZT9qyARN1jw7WrcXZ
```

## Soft Derivation from Public Key

Note: We use addresses here because Subkey does not derive paths from a public key (AFAIK).

```
> subkey inspect 5HjEC5U39iEJ71TDHLS7Tmzjsb412jNRkk4QPSz5DX18UQBu/0
Public Key URI `5HjEC5U39iEJ71TDHLS7Tmzjsb412jNRkk4QPSz5DX18UQBu/0` is account:
  Network ID/version: substrate
  Public key (hex):   0xa68af0a0883030efb2d4eb7e7adb9a8de684508de90dc4b4a4f84232ff83ff3a
  SS58 Address:       5Fq55aj4dfriNvF8wmVoCJo5NiTx8xpN4XkpDEaZQKTgP5mc

> subkey inspect 5HjEC5U39iEJ71TDHLS7Tmzjsb412jNRkk4QPSz5DX18UQBu/1
Public Key URI `5HjEC5U39iEJ71TDHLS7Tmzjsb412jNRkk4QPSz5DX18UQBu/1` is account:
  Network ID/version: substrate
  Public key (hex):   0x1e901acf62f90c32e10518063b30600a1bdf010d399ce6fb06350042b1b6af3c
  SS58 Address:       5Ckn6rXjTmsYABZSLcHQLEyfzegscFfZT9qyARN1jw7WrcXZ
```
