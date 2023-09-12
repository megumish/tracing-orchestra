# tracing-orchestra

Have you ever found it tedious to attach `#[tracing::instrument]` to each function one by one? That's where `#[tracing_orchestra::instrument]` comes in!

`#[tracing_orchestra::orchestra]` allows you to attach what would have been `#[tracing::instrument]` on each function, to the `impl` block instead.

## Installation

```toml
[dependencies]
tracing-orchestra = "0.1"
```

## How to use

```rust
use tracing_orchestra::orchestra;

#[orchestra]
impl Foo {
    fn bar() {}
    fn baz() {}
}
```

## Features

- [x] Batch assignment of `#[tracing::instrument]` to functions.
- [ ] Set default values for `#[tracing::instrument]`.
  - [x] Set default values for `#[tracing::instrument]`.
  - [ ] Set values to override for `#[tracing::instrument]`.
- [ ] Automatic implementation of Debug and Display
  - [ ] Setup using the standard Derive macro for Debug and Display.
  - [ ] Configuration when you want to hide some values during tracing (e.g., sensitive information such as access tokens)

## Why "orchestra"?

Because there are a lot of instruments.

## License

MIT
