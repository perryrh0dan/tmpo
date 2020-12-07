# Development

## Contributing

### Commit Message Header

``` text
<type>(<scope>): <short summary>
  │       │             │
  │       │             └─⫸ Summary in present tense. Not capitalized. No period at the end.
  │       │
  │       └─⫸ Commit Scope: file or module
  │
  └─⫸ Commit Type: build|ci|docs|feat|fix|perf|refactor|style|test
```

## Build

### Binary

Tmpo is using [cross](https://github.com/rust-embedded/cross) to build cross platform.

``` bash
cross build --target x86_64-pc-windows-gnu
cross build --release --target x86_64-pc-windows-gnu
```

## Testing

## Benchmark

### Flamegraph

- Follow [this](https://github.com/flamegraph-rs/flamegraph) steps to install flamegraph
- Run following command
``` bash
cargo flamegraph --dev --bin=tmpo init thomas
```
## Codecoverage

- Install [grcov](https://github.com/mozilla/grcov)
- Run ./scripts/coverage.sh
