# ChainDemocracy

ChainDemocracy is a decentralized application built on the Solana blockchain using Rust. 
It aims to improve the modern e-voting system to prevent fraud in political elections.

## Pre Requisites
- Install the Local Development Environment using this site:
https://docs.solana.com/getstarted/local

- Install Rust Environment using this site:
https://docs.solana.com/getstarted/rust

## Compile

To compile the program with cargo:
```sh
cargo build-bpf
```

## Test

### To deploy ChainDemocracy program:
First of all open the CLI and write:

```sh
solana-test-validator
```
and then open another CLI and write:

```sh
solana program deploy target/deploy/chain_democracy.so
```
Save the Program ID and modify the main of all the scripts:

```sh
const chainDemocracyProgramId = new web3.PublicKey('*new Public Key*')
```

### To run the scripts:

- Before running any command, install all the dependencies:

  ```sh
  cd Script/src
  npm install
  ```

- Install @solana/web3.js:
  ```sh
  npm install --save @solana/web3.js
  ```

- Install @project-serum/borsh
  ```sh
  npm install @project-serum/borsh
  ```

- Install dotenv
  ```sh
  npm install dotenv
  ```

Run the script in this order:

```sh
npm run newElection
```
```sh
npm run addCandidate
```
```sh
npm run vote
```
