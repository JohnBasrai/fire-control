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

Built and tested on Rust 1.83.0. Code is expected to compile on Rust 1.70+, but the lock file format requires Rust 1.82+ unless regenerated.

```bash
cargo build --release
````

---

## ▶️ Run the Server

```bash
# Default log level is warn.
RUST_LOG=debug cargo run --bin fire-control
```

It listens on port `8124` and logs events via `tracing`.

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

### 🖨️ Sample Server Output

When the test driver runs, the server prints:

```

INFO  Starting fire-control... port=8124
INFO  🚪 Listening on 0.0.0.0:8124
INFO  🔌 Accepted connection peer=127.0.0.1:39272
DEBUG Received raw line trimmed=10
INFO  ➡️ Received command cmd=Fire(10.0)
INFO  ⏳ Scheduled new firing command delay\_secs=10.0
DEBUG Received raw line trimmed=3
INFO  ➡️ Received command cmd=Fire(3.0)
INFO  🆕 Replacing existing firing command delay\_secs=3.0
DEBUG Received raw line trimmed=-1
INFO  ➡️ Received command cmd=Cancel
INFO  ⛔ Command cancelled
DEBUG Received raw line trimmed=hello
WARN  ⚠️ Invalid command: Failed to parse input as float: "hello"
DEBUG Received raw line trimmed=0
WARN  ⚠️ Invalid command: Delay must be positive or -1 to cancel
DEBUG Received raw line trimmed=5
INFO  ➡️ Received command cmd=Fire(5.0)
INFO  ⏳ Scheduled new firing command delay\_secs=5.0
INFO  🔌 Connection closed peer=127.0.0.1:39272
firing now!
INFO  🚀 Firing now! delay\_secs=5.0

```

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

