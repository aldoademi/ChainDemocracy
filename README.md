# ChainDemocracy

ChainDemocracy is a decentralized application build on Solana blockchain using Rust.
It aims to improve the modern e-voting system to prevent fraud into elections.

## Pre Requisites
- Install the Local Development Environment using this site:
https://docs.solana.com/getstarted/local

- Install Rust Environment using this site:
https://docs.solana.com/getstarted/rust

- Before running any command, install all the dependencies:

  ```sh
  npm install
  ```

- Install @solana/web3.js:
  ```sh
  npm install --save @solana/web3.js
  ```

## Compile

To compile the program with cargo:
```sh
cargo build-bpf
```

## Test

To deploy ChainDemocracy program:
```sh
solana program deploy target/deploy/chain_democracy.so
```
Save the PublicKey and modify the main of all the scripts:

```sh
const chainDemocracyProgramId = new web3.PublicKey('*new Public Key*')
```

To run the scripts:

```sh
npm run newElectionAccount
```
```sh
npm run createCandidateList
```
```sh
npm run createCandidateAccount
```
