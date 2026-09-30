# jco: a resource returned by an async import aliases an existing handle

`outer` calls `inner`'s async `api.run`. Inside it, `inner` gets a `thing` from the sync
host import `root()`, then awaits the async host import `make("b")`:

```rust
let a = root();
let b = make("b".into()).await;
format!("{} {}", a.name(), b.name())
```

**Expected:** `root b`. Wasmtime 49.0.1 prints this for the same `composed.wasm`, run from
a small Rust embedder that implements `host` (not included here).

**Actual (jco 1.35.0):** the handle `inner` gets back from `make` points at `root`'s
object, so `b.name()` runs on `root`. Dropping both handles then fails:

```
name() called on root
name() called on root
RuntimeError: unknown handle index 3
```

## Run

Needs Rust, Node, `wasm-tools` and `wac`:

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-tools wac-cli
```

Then:

```sh
npm install
./build.sh
node run.mjs
```

## What it depends on

Each of these changes makes it print `root b`:

- calling `inner`'s `api.run` directly from JS instead of through `outer`;
- getting `a` from `make` as well, so both handles come from the async import;
- making `make` a sync function.

Adding `--async-mode jspi` to the `jco transpile` command in `build.sh` fails the same way.

Tested with jco 1.35.0, wit-bindgen 0.62.0, wasm-tools 1.258.0, wac 0.11.0,
Node 24.20.0, Rust 1.98.1 and Wasmtime 49.0.1.
