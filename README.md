# ZK Benchmark

This is a small tool that run Circom and Noir on a set of examples from the tests folder.



## Getting started

### Prerequisites

1. Install wget (macOS with Homebrew: `brew install wget`)
2. Install b2sum (macOS with Homebrew: `brew install blake2`)
3. Install [Yarn](https://classic.yarnpkg.com/lang/en/docs/install/)
4. Install [Rust](https://www.rust-lang.org/tools/install)
5. Install [Nargo](https://noir-lang.org/docs/getting_started/installation/)
6. Install [Circom](https://docs.circom.io/getting-started/installation/#installing-circom)

### Benchmarking

1. `git clone https://github.com/noir-lang/zk_bench.git`
2. `cd zk_bench`
3. `yarn`
4. `bash download_ptau.sh`
5. `cargo run`

### Tests

Tests are using [Circom lib](https://github.com/iden3/circomlib/) and [Noir stdlib](https://github.com/noir-lang/noir/tree/master/noir_stdlib)

- eddsa: eddsa signature verification.
- gates: AND the inputs together
- mimc: mimc hash of the input
- pedersen: pedersen hash of the input
- poseidon: poseidon hash of the input
- sha256: sha256 hash of the input


## Support

Need help? Join the [Noir Discord](https://discord.gg/JtqzkdeQ6G) or reach out on [Twitter](https://twitter.com/NoirLang).

## Contributing

We welcome contributions! Check out the [contributing guidelines](./CONTRIBUTING.md) for more info.
