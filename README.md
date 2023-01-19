# serde-env &emsp; [![Build Status]][actions] [![Latest Version]][crates.io]

[Build Status]: https://img.shields.io/github/actions/workflow/status/Xuanwo/serde-env/ci.yml?branch=main
[actions]: https://github.com/Xuanwo/serde-env/actions?query=branch%3Amain
[Latest Version]: https://img.shields.io/crates/v/serde-env.svg
[crates.io]: https://crates.io/crates/serde-env

Deserialize env into structs via serde

## Quick Start

```rust
use serde::Deserialize;
use serde_env::from_env;

#[derive(Debug, Deserialize)]
struct Cargo {
    home: String,
}

#[derive(Debug, Deserialize)]
struct Test {
    home: String,
    cargo: Cargo,
}

fn main() {
    let t: Test = from_env().expect("deserialize from env");

    assert!(!t.home.is_empty());
    assert!(!t.cargo.home.is_empty());
    println!("{:?}", t)
}
```

## Contributing

Check out the [CONTRIBUTING.md](./CONTRIBUTING.md) guide for more details on getting started with contributing to this project.

## Getting help

Submit [issues](https://github.com/Xuanwo/serde-env/issues/new/choose) for bug report or asking questions in [discussion](https://github.com/Xuanwo/serde-env/discussions/new?category=q-a).

## Acknowledgment

This project is highly inspired by [envy](https://github.com/softprops/envy)

#### License

<sup>
Licensed under <a href="./LICENSE">Apache License, Version 2.0</a>.
</sup>
