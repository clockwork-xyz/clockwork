# Cronos - Decentralized task scheduler for Solana

Multi-repo for [Cronos](https://www.cronos.so/) comprised of SDKs, playground projects, and its program implementation.

## Packages

| Package | Description | Version | Docs |
| :-- | :-- | :--| :-- |
| `cronos-sdk` | Rust SDK for building with Cronos | ![crates](https://img.shields.io/crates/v/cronos-sdk?color=blue) | [![Docs.rs](https://docs.rs/cronos-sdk/badge.svg)](https://docs.rs/cronos-sdk/0.0.1/cronos_sdk)
| `@cronos-so/web` | Typescript SDK for building with Cronos | [![npm](https://img.shields.io/npm/v/@cronos-so/web.svg?color=blue)](https://www.npmjs.com/package/@cronos-so/web)  | [GitBook](https://docs.cronos.so/integrate/user-instructions)
| `bot` | Workers that listen to Cronos's program state and execute pending tasks
| `cli` | CLI to support scheduling tasks and managing daemons with Cronos
| `programs` | Collection of Anchor programs that implement Cronos
