<div align="center">
  <img height="170" src="https://user-images.githubusercontent.com/8634334/167159164-17b3b09a-ed1e-4768-b405-af9d423192c9.png?raw=true" />

  <h1>Cronos</h1>

  <p>
    <strong>Automation infrastructure for Solana</strong>
  </p>

  <p>
    <a href="https://github.com/cronos-so/cronos/actions/workflows/code-scan.yaml"><img alt="code scan" src="https://github.com/cronos-so/cronos/actions/workflows/code-scan.yaml/badge.svg?branch=main" /></a>
    <a href="https://discord.com/channels/889725689543143425"><img alt="Discord Chat" src="https://img.shields.io/discord/889725689543143425?color=blueviolet" /></a>
    <a href="https://www.gnu.org/licenses/agpl-3.0.en.html"><img alt="License" src="https://img.shields.io/github/license/cronos-so/cronos?color=turquoise" /></a>
  </p>

  <h4>
    <a href="https://cronos.so/">Home</a>
    <span> | </span>
    <a href="https://docs.cronos.so">Docs</a>
    <span> | </span>
    <a href="https://twitter.com/cronos_so">Twitter</a>
  </h4>  
</div>


## Deployments

| Program | Address| Devnet | Testnet | Mainnet Beta |
| ------- | ------ | ------ | ------- | ------------ |
| Network | `7dgApA7ixgamh8TRJDVqhsPVDxn4RVQgW93xgcSqpzWQ` | [v0.2.0](https://explorer.solana.com/address/7dgApA7ixgamh8TRJDVqhsPVDxn4RVQgW93xgcSqpzWQ?cluster=devnet) | Soon | Soon |
| Scheduler | `7XgnJEERdx6SXiXbv1TqSe1XCYCa81BVC5UYRA1mnN1o` | [v0.2.0](https://explorer.solana.com/address/7XgnJEERdx6SXiXbv1TqSe1XCYCa81BVC5UYRA1mnN1o?cluster=devnet) | Soon | Soon |


## Notes

- Cronos is under active development. All interfaces and implementations are subject to change.
- Smart contracts are automatically scanned by [Sec3](https://www.sec3.dev/)'s auto-auditing software, but have not been reviewed by a paid auditing firm.
- Use at your own risk.

## Plugin

To run the Cronos plugin on your Solana validator, you can either `cargo build` from scratch or install the pre-built binary:
```sh
curl -s https://api.github.com/repos/cronos-so/cronos/releases/latest | grep "cronos-geyser-plugin-release-x86_64-unknown-linux-gnu.tar" | cut -d : -f 2,3 | tr -d \" | wget -qi -
tar -xjvf cronos-geyser-plugin-release-x86_64-unknown-linux-gnu.tar.bz2
rm cronos-geyser-plugin-release-x86_64-unknown-linux-gnu.tar.bz2
```


Next, create a new keypair for signing Cronos txs. The responsbilities of being a worker currently include the processing of scheduled tasks and the rotating the worker pool. Cronos workers may be expected to perform other jobs in the near future. We recommend loading this keypair with a small amount of SOL (~0.01 ◎). You will be compensated for lamports spent by the programs invoked in a task. Automation fees (rewards) are implemented and will be enabled soon.
```sh
solana-keygen new -o cronos-worker-keypair.json
```

Then, setup the plugin config file in a folder where your startup script can reference it. Note, the `libpath` and `keypath` values should point to the binary and keypair mentioned in the steps above.
```js
{
  "libpath": "/home/sol/cronos-geyser-plugin-release/lib/libcronos_plugin.so",
  "keypath": "/home/sol/cronos-worker-keypair.json",
  "rpc_url": "http://127.0.0.1:8899",
  "slot_timeout_threshold": 150,
  "worker_threads": 10
}
```

Finally, add an additional line to your startup script to run your validator with the Cronos plugin (often located at `/home/sol/bin/validator.sh`):
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



## Token

Airdrops and staking instructions coming soon... ⚙️




