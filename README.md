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

## Notes

- Cronos is under active development. All interfaces and implementations are subject to change.
- Smart contracts are automatically scanned by Soteria's [auditing software](https://www.soteria.dev/software), but have not been reviewed by a paid auditing firm.
- Use at your own risk.

## Deployments

| Program                                                           | Devnet                                         | Mainnet Beta                                   |
| ----------------------------------------------------------------- | ---------------------------------------------- | ---------------------------------------------- |
| [v0.1.8](https://github.com/cronos-so/cronos/releases/tag/v0.1.8) | `CronpZj5NbHj2Nb6WwEtf6A9anty9JfEQ1RnGoshQBaW` | `CronpZj5NbHj2Nb6WwEtf6A9anty9JfEQ1RnGoshQBaW` |

## Packages

| Package              | Description                                  | Version                                                                  | Docs                                                                                           |
| :------------------- | :------------------------------------------- | :----------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------- |
| `cronos-cli`         | Command line interface                       | ![crates](https://img.shields.io/crates/v/cronos-cli?color=blue)         | [GitBook](https://docs.cronos.so/about/cli)                                                    |
| `cronos-cron`        | Solana-safe cron expression parser           | ![crates](https://img.shields.io/crates/v/cronos-cron?color=blue)        | [![Docs.rs](https://docs.rs/cronos-cron/badge.svg)](https://docs.rs/cronos-cron)               |
| `cronos-plugin`      | Geyser plugin for Solana validators          | ![crates](https://img.shields.io/crates/v/cronos-plugin?color=blue)      | [GitBook](https://docs.cronos.so/about/architecture/bots)                                      |
| `cronos-network`     | Staking program for nodes running the plugin | ![crates](https://img.shields.io/crates/v/cronos-network?color=blue)     | [![Docs.rs](https://docs.rs/cronos-network/badge.svg)](https://docs.rs/cronos-network)         |
| `cronos-healthcheck` | Timer program to measure network liveness    | ![crates](https://img.shields.io/crates/v/cronos-healthcheck?color=blue) | [![Docs.rs](https://docs.rs/cronos-healthcheck/badge.svg)](https://docs.rs/cronos-healthcheck) |
| `cronos-scheduler`   | Queue scheduling program                     | ![crates](https://img.shields.io/crates/v/cronos-scheduler?color=blue)   | [![Docs.rs](https://docs.rs/cronos-scheduler/badge.svg)](https://docs.rs/cronos-scheduler)     |
| `cronos-sdk`         | Cronos developer kit                         | ![crates](https://img.shields.io/crates/v/cronos-sdk?color=blue)         | [![Docs.rs](https://docs.rs/cronos-sdk/badge.svg)](https://docs.rs/cronos-sdk)                 |
| `cronos-telemetry`   | Observability and monitoring service         | ![crates](https://img.shields.io/crates/v/cronos-telemetry?color=blue)   | Coming soon!                                                                                   |
| `cronos-tests`       | Stress testing suite                         | ![crates](https://img.shields.io/crates/v/cronos-tests?color=blue)       | Coming soon!                                                                                   |
