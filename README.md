# tracing-orchestra

Have you ever found it tedious to attach `#[tracing::instrument]` to each function one by one? That's where `#[tracing_orchestra::instrument]` comes in!

`#[tracing_orchestra::orchestra]` allows you to attach what would have been `#[tracing::instrument]` on each function, to the `impl` block instead.

## Installation

```toml
[dependencies]
tracing-orchestra = "0.2"
```

## How to use

```rust
use tracing_orchestra::orchestra;

// default instrument value is here!
#[orchestra::(level = "trace")]
impl Foo {
    fn bar() {}
    // override tracing::instrument
    #[tracing::instrument(level = "info")]
    fn baz() {}
}
```

## Features

- [x] Batch assignment of `#[tracing::instrument]` to functions.
- [x] Set default values for `#[tracing::instrument]`.
  - [x] Set default values for `#[tracing::instrument]`.
  - [x] Set values to override for `#[tracing::instrument]`.
- [ ] Automatic implementation of Debug and Display
  - [ ] Setup using the standard Derive macro for Debug and Display.
  - [ ] Configuration when you want to hide some values during tracing (e.g., sensitive information such as access tokens)

## Why "orchestra"?

Because there are a lot of instruments.

## License

MIT
