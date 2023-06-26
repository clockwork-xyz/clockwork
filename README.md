<div align="center">
  <h1>Clockwork</h1>

  <p>
    <strong>Solana automation engine</strong>
  </p>

  <p>
    <a href="https://github.com/clockwork-xyz/clockwork/actions/workflows/code-scan.yaml"><img alt="code scan" src="https://github.com/clockwork-xyz/clockwork/actions/workflows/code-scan.yaml/badge.svg?branch=main" /></a>
    <a href="https://discord.com/channels/889725689543143425"><img alt="Discord Chat" src="https://img.shields.io/discord/889725689543143425?color=blueviolet" /></a>
    <a href="https://www.gnu.org/licenses/agpl-3.0.en.html"><img alt="License" src="https://img.shields.io/github/license/clockwork-xyz/clockwork?color=turquoise" /></a>
  </p>

  <h4>
    <a href="https://clockwork.xyz/">Home</a>
    <span> | </span>
    <a href="https://docs.clockwork.xyz">Docs</a>
    <span> | </span>
    <a href="https://twitter.com/clockwork_xyz">Twitter</a>
  </h4>  
</div>

# Deployments

| Program | Address| Devnet | Mainnet |
| ------- | ------ | ------ | ------- |
| Network | `F8dKseqmBoAkHx3c58Lmb9TgJv5qeTf3BbtZZSEzYvUa` | [v2.0.15](https://explorer.solana.com/address/F8dKseqmBoAkHx3c58Lmb9TgJv5qeTf3BbtZZSEzYvUa) | [v2.0.0](https://explorer.solana.com/address/F8dKseqmBoAkHx3c58Lmb9TgJv5qeTf3BbtZZSEzYvUa) |
| Thread v2 | `CLoCKyJ6DXBJqqu2VWx9RLbgnwwR6BMHHuyasVmfMzBh` | [v2.0.18](https://explorer.solana.com/address/CLoCKyJ6DXBJqqu2VWx9RLbgnwwR6BMHHuyasVmfMzBh?cluster=devnet) | [v2.0.17](https://explorer.solana.com/address/CLoCKyJ6DXBJqqu2VWx9RLbgnwwR6BMHHuyasVmfMzBh) |
| Thread v1 | `3XXuUFfweXBwFgFfYaejLvZE4cGZiHgKiGfMtdxNzYmv` | [v1.4.2](https://explorer.solana.com/address/3XXuUFfweXBwFgFfYaejLvZE4cGZiHgKiGfMtdxNzYmv?cluster=devnet) | [v1.4.2](https://explorer.solana.com/address/3XXuUFfweXBwFgFfYaejLvZE4cGZiHgKiGfMtdxNzYmv) |

# SDKs
| Language | Description  | Lib  | Examples |
| ----------- | -------- | ---- | -------- |
| Anchor |  Anchor bindings for Solana programs.  | [crates.io](https://crates.io/crates/clockwork-sdk) | [See Example Repo](https://github.com/clockwork-xyz/examples)
| Rust | Rust bindings for clients.  | [crates.io](https://crates.io/crates/clockwork-client) | [See Example Repo](https://github.com/clockwork-xyz/examples)
| Typescript | Typescript bindings for clients and frontends.  | [npm](https://www.npmjs.com/package/@clockwork-xyz/sdk) | [Explorer](https://github.com/clockwork-xyz/explorer)

# Notes
- Clockwork is under active development. All interfaces and implementations are subject to change. 
- Official program deployments to Solana mainnet are secured by a 2-of-2 [multisig](https://v3.squads.so/info/7gqj7UgvKgHihyPsXALW8QKJ3gUTEaLeBYwWbAtZhoCq) and managed by the core team of software maintainers. 
- To deploy a worker node on mainnet or devnet, please [install](#deploying-a-worker) the Clockwork geyser plugin on your Solana validator or RPC node and request an earlybird token delegation in the workernet channel [on Discord](https://discord.gg/mwmFtU5BtA).
- Occasionally, a new software release may change the state schema and require users to migrate to a new program. These releases will be marked by a new major version upgrade (e.g. `v2.x`, `v3.x`, etc.). 
- The smart-contracts in this repository are automatically scanned by [Sec3's](https://www.sec3.dev/) auto-auditing software and are currently being reviewed by the team at [Ottersec](https://osec.io/). Their audit report is in progress and will be published soon. 

# Getting Started
- ["I am a developer, and I want to build a program on localnet"](#local-development)
- ["I am a node operator, and I want to deploy a Clockwork worker"](#deploying-a-worker)

# Local Development

#### 1. Install clockwork-cli.
If you are on linux, you might need to run this:
```sh
sudo apt-get update && sudo apt-get upgrade && sudo apt-get install -y pkg-config build-essential libudev-dev libssl-dev
```
Install with cargo:
```sh
cargo install -f --locked clockwork-cli
```

#### 2. Run a localnet node.
```sh
clockwork localnet
```

#### 3. Stream program logs.
```sh
solana logs --url localhost
```

# Guides & Examples
- We have ready to run examples apps: https://github.com/clockwork-xyz/examples.
- If you are looking for walkthough, take a look at the docs: https://docs.clockwork.xyz/developers/guides.
- If you have a certain use case you would like to discuss, we are happy to [help](https://discord.com/channels/889725689543143425/1029516796304306247).

---

# Deploying a worker
> If you just want to test your smart contracts on localnet, check the previous section.

If you are a node operator looking to deploy the clockwork plugin, please talk to us for a smooth onboarding. Here's a one pager on how to be part of the automation network: https://docs.clockwork.xyz/workernet/deploying-a-worker.

## Common Errors
Please refer to the [FAQ](https://docs.clockwork.xyz/developers/faq).

## Questions
Come build with us and ask questions on [Discord](https://discord.gg/epHsTsnUre)!
