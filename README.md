<div align="center">
  <img height="170" src="https://user-images.githubusercontent.com/8634334/167159164-17b3b09a-ed1e-4768-b405-af9d423192c9.png?raw=true" />

  <h1>Clockwork</h1>

  <p>
    <strong>Automation engine for the Solana blockchain</strong>
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


## Deployments

| Program | Address| Devnet | Mainnet |
| ------- | ------ | ------ | ------- |
| Network | `F8dKseqmBoAkHx3c58Lmb9TgJv5qeTf3BbtZZSEzYvUa` | [v1.3.15](https://explorer.solana.com/address/F8dKseqmBoAkHx3c58Lmb9TgJv5qeTf3BbtZZSEzYvUa?cluster=devnet) | [v1.3.15](https://explorer.solana.com/address/F8dKseqmBoAkHx3c58Lmb9TgJv5qeTf3BbtZZSEzYvUa) |
| Thread | `3XXuUFfweXBwFgFfYaejLvZE4cGZiHgKiGfMtdxNzYmv` | [v1.3.15](https://explorer.solana.com/address/3XXuUFfweXBwFgFfYaejLvZE4cGZiHgKiGfMtdxNzYmv?cluster=devnet) | [v1.3.15](https://explorer.solana.com/address/3XXuUFfweXBwFgFfYaejLvZE4cGZiHgKiGfMtdxNzYmv) |


## Notes

- Clockwork is under active development. All interfaces and implementations are subject to change.
- Smart contracts are automatically scanned by [Sec3](https://www.sec3.dev/)'s auto-auditing software, but have not been reviewed by a paid auditing firm.
- Use at your own risk.

## Getting Started
- ["I am a validator and I want to deploy the Clockwork Engine"](#deploying-a-worker)
- ["I don't have a validator or I just want to do some tests on localhost"](#local-development)


## Local Development

Download the source code:
```sh
git clone https://github.com/clockwork-xyz/clockwork
cd clockwork
```

The `main` branch is under active development and subject to bugs. To work with a stable version, checkout a release branch:
```sh
git describe --tags `git rev-list --tags --max-count=1`
git checkout tags/...
```

Build the repo:
```sh
./scripts/build-all.sh .
export PATH=$PWD/bin:$PATH
```

Start a local node for development:
```sh
clockwork localnet
```

To stream program logs:
```sh
solana logs --url localhost
```


## Deploying a worker

To run the Clockwork plugin on your Solana validator, you can either build from scratch (shown above) or install the pre-built binary:
```sh
curl -s https://api.github.com/repos/clockwork-xyz/clockwork/releases/latest | grep "clockwork-geyser-plugin-release-x86_64-unknown-linux-gnu.tar" | cut -d : -f 2,3 | tr -d \" | wget -qi -
tar -xjvf clockwork-geyser-plugin-release-x86_64-unknown-linux-gnu.tar.bz2
rm clockwork-geyser-plugin-release-x86_64-unknown-linux-gnu.tar.bz2
```

Next, create a new keypair for signing Clockwork txs. Load this keypair with a small amount of SOL (~0.01 ◎). You will be compensated for lamports spent by the tasks your worker executes. Automation fees (rewards) are implemented and will soon be enabled.
```sh
solana-keygen new -o clockwork-worker-keypair.json
```

Then, setup the plugin config file in a folder where your validator startup script can reference it. Note, the `libpath` and `keypath` values should point to the binary and keypair mentioned in the steps above.
```js
{
  "libpath": "/home/sol/clockwork-geyser-plugin-release/lib/libclockwork_plugin.so",
  "keypath": "/home/sol/clockwork-worker-keypair.json",
  "rpc_url": "http://127.0.0.1:8899",
  "transaction_timeout_threshold": 150,
  "thread_count": 10,
  "worker_id": 0,  // Set this to your worker ID!
}
```

Finally, add an additional line to your startup script to run your validator with the Clockwork plugin (often located at `/home/sol/bin/validator.sh`):
```sh
#!/bin/bash

exec solana-validator \
    --identity /home/sol/validator-keypair.json \
    --known-validator dv1ZAGvdsz5hHLwWXsVnM94hWf1pjbKVau1QVkaMJ92 \
    --known-validator dv2eQHeP4RFrJZ6UeiZWoc3XTtmtZCUKxxCApCDcRNV \
    --known-validator dv4ACNkpYPcE3aKmYDqZm9G5EB3J4MRoeE7WNDRBVJB \
    --known-validator dv3qDFk1DTF36Z62bNvrCXe9sKATA6xvVy6A798xxAS \
    --only-known-rpc \
    --full-rpc-api \
    --no-voting \
    --ledger /mnt/ledger \
    --accounts /mnt/accounts \
    --log /home/sol/solana-rpc.log \
    --rpc-port 8899 \
    --rpc-bind-address 0.0.0.0 \
    --dynamic-port-range 8000-8020 \
    --entrypoint entrypoint.devnet.solana.com:8001 \
    --entrypoint entrypoint2.devnet.solana.com:8001 \
    --entrypoint entrypoint3.devnet.solana.com:8001 \
    --entrypoint entrypoint4.devnet.solana.com:8001 \
    --entrypoint entrypoint5.devnet.solana.com:8001 \
    --expected-genesis-hash EtWTRABZaYq6iMfeYKouRu166VU2xqa1wcaWoxPkrZBG \
    --wal-recovery-mode skip_any_corrupted_record \
    --limit-ledger-size \
    
    # Add this line! 👇🏼
    --geyser-plugin-config /home/sol/geyser-plugin-config.json
```

Now simply restart your validator however you normally would!

