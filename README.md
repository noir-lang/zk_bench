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

It then benchmarks tests in the `./tests/` directory. Note that some of the tests might take minutes to run.

### Tests

Tests are using [Circom lib](https://github.com/iden3/circomlib/) and [Noir stdlib](https://github.com/noir-lang/noir/tree/master/noir_stdlib)

- eddsa: eddsa signature verification.
- gates: AND the inputs together
- mimc: mimc hash of the input
- pedersen: pedersen hash of the input
- poseidon: poseidon hash of the input
- sha256: sha256 hash of the input



## Example results

Example results from a trial run on an M2 Macbook Air:

```
Testing gates with circom:
        5 constraints
        setup generated in 489ms
        compiled in 31ms
        execution in 322ms
        prove in 392ms
Testing gates with Noir:
        11 constraints (5 ACIR opcodes)
        setup generated in 0ms
        compiled in 238ms
        execution in 317ms
        prove in 395ms
```

```
Testing sha256_30 with circom:
        904020 constraints
        setup generated in 717844ms
        compiled in 39001ms
        execution in 2157ms
        prove in 20229ms
Testing sha256_30 with Noir:
        120086 constraints (90 ACIR opcodes)
        setup generated in 0ms
        compiled in 272ms
        execution in 329ms
        prove in 3039ms
```

```
Testing poseidon_30 with circom:
        7200 constraints
        setup generated in 22217ms
        compiled in 960ms
        execution in 302ms
        prove in 851ms
Testing poseidon_30 with Noir:
        64955 constraints (64920 ACIR opcodes)
        setup generated in 0ms
        compiled in 6248ms
        execution in 6165ms
        prove in 8183ms
```

```
Testing sha256 with circom:
        30134 constraints
        setup generated in 49852ms
        compiled in 1958ms
        execution in 386ms
        prove in 1140ms
Testing sha256 with Noir:
        38799 constraints (3 ACIR opcodes)
        setup generated in 0ms
        compiled in 235ms
        execution in 323ms
        prove in 1769ms
```

```
Testing mimc with circom:
        728 constraints
        setup generated in 1369ms
        compiled in 28ms
        execution in 247ms
        prove in 467ms
Testing mimc with Noir:
        830 constraints (824 ACIR opcodes)
        setup generated in 0ms
        compiled in 484ms
        execution in 578ms
        prove in 695ms
```

```
Testing poseidon with circom:
        240 constraints
        setup generated in 1534ms
        compiled in 540ms
        execution in 283ms
        prove in 418ms
Testing poseidon with Noir:
        2170 constraints (2164 ACIR opcodes)
        setup generated in 0ms
        compiled in 432ms
        execution in 510ms
        prove in 691ms
```

```
Testing pedersen with circom:
        3 constraints
        setup generated in 504ms
        compiled in 42ms
        execution in 247ms
        prove in 397ms
Testing pedersen with Noir:
        28742 constraints (1 ACIR opcodes)
        setup generated in 0ms
        compiled in 240ms
        execution in 335ms
        prove in 1079ms
```

```
Testing eddsa with circom:
        7554 constraints
        setup generated in 17266ms
        compiled in 939ms
        execution in 315ms
        prove in 702ms
Testing eddsa with Noir:
        24472 constraints (21645 ACIR opcodes)
        setup generated in 0ms
        compiled in 6748ms
        execution in 6746ms
        prove in 7670ms
```

```
Testing pedersen_30 with circom:
        90 constraints
        setup generated in 771ms
        compiled in 86ms
        execution in 239ms
        prove in 398ms
Testing pedersen_30 with Noir:
        30424 constraints (30 ACIR opcodes)
        setup generated in 0ms
        compiled in 253ms
        execution in 342ms
        prove in 1082ms
```



## Support

Need help? Join the [Noir Discord](https://discord.gg/JtqzkdeQ6G) or reach out on [Twitter](https://twitter.com/NoirLang).

## Contributing

We welcome contributions! Check out the [contributing guidelines](./CONTRIBUTING.md) for more info.
