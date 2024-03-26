# ZK Benchmark

This is a small tool that run Circom and Noir on a set of examples from the tests folder.



## Getting started

You need to have both Noir and Circom properly installed. Then you need to provide binary paths in circom.rs and noir.rs

### Tests

Tests are using [Circom lib](https://github.com/iden3/circomlib/) and [Noir stdlib](https://github.com/noir-lang/noir/tree/master/noir_stdlib)

- eddsa: eddsa signature verification. Not enabled yet
- gates: AND the inputs together
- mimc: mimc hash of the input
- pedersen: pedersen hash of the input
- poseidon: poseidon hash of the input
- sha256: sha256 hash of the input


## Support

Need help? Join the [Noir Discord](https://discord.gg/JtqzkdeQ6G) or reach out on [Twitter](https://twitter.com/NoirLang).

## Contributing

We welcome contributions! Check out the [contributing guidelines](./CONTRIBUTING.md) for more info.
