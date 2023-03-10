<div align="center">
  <h1>Clockwork</h1>

  <p>
    <strong>Automation engine for Solana programs</strong>
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
| Network | `F8dKseqmBoAkHx3c58Lmb9TgJv5qeTf3BbtZZSEzYvUa` | [v2.0.0](https://explorer.solana.com/address/F8dKseqmBoAkHx3c58Lmb9TgJv5qeTf3BbtZZSEzYvUa) | [v2.0.0](https://explorer.solana.com/address/F8dKseqmBoAkHx3c58Lmb9TgJv5qeTf3BbtZZSEzYvUa) |
| Thread v2 | `CLoCKyJ6DXBJqqu2VWx9RLbgnwwR6BMHHuyasVmfMzBh` | [v2.0.0](https://explorer.solana.com/address/CLoCKyJ6DXBJqqu2VWx9RLbgnwwR6BMHHuyasVmfMzBh?cluster=devnet) | [v2.0.0](https://explorer.solana.com/address/CLoCKyJ6DXBJqqu2VWx9RLbgnwwR6BMHHuyasVmfMzBh) |
| Thread v1 | `3XXuUFfweXBwFgFfYaejLvZE4cGZiHgKiGfMtdxNzYmv` | [v1.4.2](https://explorer.solana.com/address/3XXuUFfweXBwFgFfYaejLvZE4cGZiHgKiGfMtdxNzYmv?cluster=devnet) | [v1.4.2](https://explorer.solana.com/address/3XXuUFfweXBwFgFfYaejLvZE4cGZiHgKiGfMtdxNzYmv) |

# SDKs
| Language | Description  | Lib  | Examples |
| ----------- | -------- | ---- | -------- |
| Anchor |  Anchor bindings for Solana programs.  | [crates.io](https://crates.io/crates/clockwork-sdk) | [See Example Repo](https://github.com/clockwork-xyz/examples)
| Rust | Rust bindings for clients.  | [crates.io](https://crates.io/crates/clockwork-client) | [See Example Repo](https://github.com/clockwork-xyz/examples)
| Typescript | Typescript bindings for clients and frontends.  | [npm](https://www.npmjs.com/package/@clockwork-xyz/sdk) | [Explorer](https://github.com/clockwork-xyz/explorer)

# Notes
- Clockwork is under active development. All interfaces and implementations are subject to change. 
- Program deployments are secured by a 2-of-2 [multisig](https://v3.squads.so/info/7gqj7UgvKgHihyPsXALW8QKJ3gUTEaLeBYwWbAtZhoCq) and managed by a core team of maintainers. 
- Solana mainnet currently has 3 independent worker node operators. To join the workernet, you can [install](#deploying-a-worker) the Clockwork plugin and request an earlybird token delegation in the workernet [Discord channel](https://discord.gg/mwmFtU5BtA).
- Occassionally, a new release may change the state schema and require users to migrate to a new program. These releases will be marked by a new major version upgrade (e.g. `v2.x`, `v3.x`, etc.). 
- The smart-contracts in this repository are automatically scanned by [Sec3's](https://www.sec3.dev/) auto-auditing software and are currently being reviewed by the team at [Ottersec](https://osec.io/). Their audit report is in progress and will be published soon. 

# Getting Started
- ["I am a developer, and I want to build a program on localnet"](#local-development)
- ["I am a node operator, and I want to deploy a Clockwork worker"](#deploying-a-worker)

# Local Development

#### 1. Download the source code.
```sh
git clone https://github.com/clockwork-xyz/clockwork
cd clockwork
```

#### 2. Checkout the latest stable release branch.
```sh
git describe --tags `git rev-list --tags --max-count=1`
git checkout tags/...
```
> ⚠️ Make sure the version of your program or client matches the version of the engine that you are running. E.g., if you are using `clockwork-sdk = 2.0.0`, you should also `git checkout tags/v2.0.0`. We use semantic versioning, but main branch is probably not what you want.


#### 3. Build the repo.
```sh
./scripts/build-all.sh .
export PATH=$PWD/bin:$PATH
```

#### 4. Start a localnet for development.
```sh
clockwork localnet
```

#### 5. Stream program logs.
```sh
solana logs --url localhost
```


# Deploying a worker

#### 1. Either build from scratch (see above) or install the pre-built binary.
```sh
curl -s https://api.github.com/repos/clockwork-xyz/clockwork/releases/latest | grep "clockwork-geyser-plugin-release-x86_64-unknown-linux-gnu.tar" | cut -d : -f 2,3 | tr -d \" | wget -qi -
tar -xjvf clockwork-geyser-plugin-release-x86_64-unknown-linux-gnu.tar.bz2
rm clockwork-geyser-plugin-release-x86_64-unknown-linux-gnu.tar.bz2
```

#### 2. Next, create a new keypair for signing Clockwork txs.
```sh
solana-keygen new -o clockwork-worker-keypair.json
```

#### 3. Load this keypair with a small amount of SOL (~0.1 ◎). 

#### 4. Register your worker and get a worker ID: 
```sh
clockwork worker create clockwork-worker-keypair.json
```


#### 5. Setup the plugin config file.
```json
{
  "libpath": "/home/sol/clockwork-geyser-plugin-release/lib/libclockwork_plugin.so",
  "keypath": "/home/sol/clockwork-worker-keypair.json",
  "rpc_url": "http://127.0.0.1:8899",
  "transaction_timeout_threshold": 150,
  "thread_count": 10,
  "worker_id": 0, 
}
```

#### 6. Configure your validator to run with the Clockwork plugin.
```sh
#!/bin/bash
exec solana-validator \
  --identity ~/validator-keypair.json \
  --rpc-port 8899 \
  --entrypoint entrypoint.devnet.solana.com:8001 \
  --no-voting \
  --full-rpc-api \
  --limit-ledger-size \
  
  # Add this line! 👇🏼
  --geyser-plugin-config ~/clockwork-geyser-config.json
```

#### 7. Restart your validator however you normally would!
```sh
sudo systemctl restart sol
```

## Common Errors
Please refer to the [FAQ](https://docs.clockwork.xyz/developers/faq).

## Questions
Come build with us and ask us questions [Discord](https://discord.gg/epHsTsnUre)!
