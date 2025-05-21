# fire-control

A satellite propulsion control simulator that receives asynchronous firing commands over TCP and triggers delayed actions. Built as part of the Umbra SWE coding exercise using async Rust.

---

## 🚀 Features

- Accepts newline-delimited commands over TCP
- Fires a propulsion system after a specified delay
- Automatically cancels or replaces pending commands
- Ignores malformed input gracefully
- Supports multiple firings per run
- Observability via `println!` and `tracing`
- Fully asynchronous using `tokio`
- Unit-tested core logic (see `controller.rs`)
- [Planned enhancements tracked via issues](https://github.com/JohnBasrai/fire-control/issues)

---

## 📦 Build

Requires Rust 1.70+ and Cargo.

```bash
cargo build --release
````

---

## ▶️ Run the Server

```bash
cargo run --bin fire-control
```

It listens on port `8124` and logs events via `tracing`.

Use `RUST_LOG=debug` to see full logs:

```bash
RUST_LOG=debug cargo run --bin fire-control
```

---

## 🧪 Test Driver

A standalone test driver sends commands over TCP and displays output:

```bash
cargo run --bin test_driver
```

This sends:

```
10
3
-1
hello
0
5
```

Expected behavior:

* `10` → scheduled
* `3` → replaces `10`
* `-1` → cancels
* `hello`, `0` → ignored
* `5` → should fire 5 seconds later

---

## 🔍 Input Format

Each line is interpreted as a command:

| Input | Meaning                                   |
| ----- | ----------------------------------------- |
| `N`   | Fire after `N` seconds (e.g. `10`, `3.5`) |
| `-1`  | Cancel any pending firing                 |
| other | Ignored, warning logged                   |

---

## ✅ Test Coverage

Core logic in `controller.rs` is covered by unit tests:

* ✅ Fires after a delay
* ✅ Cancels pending commands
* ✅ Replaces previous command
* 🟡 Multiple firings tested manually (see [Issue #6](https://github.com/JohnBasrai/fire-control/issues/6))
* 🟡 No integration test yet (see [Issue #1](https://github.com/JohnBasrai/fire-control/issues/1))

Run tests with:

```bash
cargo test
```

---

## 🔭 TODO and Roadmap

Planned enhancements are tracked in [GitHub Issues](https://github.com/JohnBasrai/fire-control/issues), including:

* Replace test driver with integration test
* Add metrics and shutdown handling
* Improve logging and observability

---

## 📁 Project Structure

```text
src/
├── main.rs          # TCP server entry point
├── controller.rs    # Core logic (handles FireCommand)
├── command.rs       # Command parser (Fire, Cancel)
├── tcp_server.rs    # Async TCP handler
└── bin/
    └── test_driver.rs   # Manual test client
```

---

## ✍️ License

MIT or Apache 2.0 — see [LICENSE](LICENSE) file if provided.

