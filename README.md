# wp-log

[![Crates.io](https://img.shields.io/crates/v/wp-log.svg)](https://crates.io/crates/wp-log)
[![Docs.rs](https://docs.rs/wp-log/badge.svg)](https://docs.rs/wp-log)

`wp-log` bundles the structured logging macros and runtime configuration helpers used inside the WarpParse platform. 6 pre-defined targets (`ctrl`, `data`, `rule`, `dfx`, `mtrc`, `kdb`) keep service logs consistent across control plane, data plane and diagnostics modules, while the configuration helpers wire those targets into `log4rs` with rolling outputs and environment-friendly defaults.

> 🇨🇳 该 crate 提供 WarpParse 平台统一的 6 域日志宏，以及针对 `log4rs` 的默认配置与序列化/反序列化能力，可直接用于落地配置文件或测试脚本。

## Highlights

- Structured macros wrap `log` so call sites stay terse but targets remain consistent.
- `LogConf` models your logging policy with full Serde support for TOML/JSON configs.
- One call to `log_init` wires rolling file + console appenders through `log4rs`.
- `PRINT_STAT` env-gated `println_mtrc!` enables lightweight metrics dumps in smoke tests.

## Quick Start

Add the dependency:

```toml
[dependencies]
wp-log = "0.1"
```

Configure and initialize logging:

```rust
use wp_log::conf::{log_init, LogConf, Output};
use wp_log::{info_ctrl, debug_data, warn_rule};

fn main() {
    let conf = LogConf {
        level: "warn,ctrl=info,data=debug".into(),
        levels: None,
        output: Output::Both,
        file: Some(wp_log::conf::FileLogConf {
            path: "./data/logs".into(),
        }),
        ..Default::default()
    };

    log_init(&conf).expect("log init");

    info_ctrl!("service starting");
    debug_data!("batch {batch_id} finished");
    warn_rule!("policy {name} falling back");
}
```

### Using TOML configuration

`LogConf` derives `Serialize`/`Deserialize`, so you can load structured config files:

```toml
# log.toml
level = "warn"               # default/root level
output = "Both"              # Console | File | Both

[levels]                      # optional per-target overrides
ctrl = "info"
data = "debug"

[file]
path = "./data/logs"         # log directory; file name derives from the binary name
```

```rust
let conf: LogConf = toml::from_str(&std::fs::read_to_string("log.toml")?)?;
log_init(&conf)?;
```

Set `PRINT_STAT=true` to make `println_mtrc!` emit lightweight counters during tests or local experiments.

## Feature Flags

- `std` (default): enables the `orion_conf` helpers used by `log_init`/`log_for_test`. Disable for `no_std` builds where only the macros are needed.

## License

Licensed under the Elastic License 2.0. See `LICENSE` (if bundled) or <https://www.elastic.co/licensing/elastic-license> for details.
