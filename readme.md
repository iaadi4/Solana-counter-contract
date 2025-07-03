# Solana Counter Smart Contract

This is a simple Solana smart contract written in Rust that maintains a counter stored on-chain. The counter can be incremented or decremented using serialized instructions.

## Features

- Stores a `u32` counter in an account
- Supports two instructions:
  - `Increment(u32)`
  - `Decrement(u32)`
- Uses Borsh for instruction and account data serialization

## License

This project is licensed under the [MIT License](./LICENSE).