# tabulardata

Safe, idiomatic Rust bindings for Apple's [TabularData](https://developer.apple.com/documentation/tabulardata) framework on macOS.

## Features

- **Swift-only framework bridge** — wraps `TabularData.DataFrame` and `AnyColumn` behind retained Swift boxes.
- **CSV read/write** — load CSV files into `DataFrame`, inspect shapes and columns, and write frames back to disk.
- **Column-oriented construction** — build frames from Rust `Column` values containing strings, integers, doubles, or booleans.
- **Summary + joins** — derive summary frames and perform left/right/full/inner joins on a column name.
- **JSON row snapshots** — inspect heterogeneous rows as `serde_json::Value` without losing unsupported summary columns.

## Requirements

- macOS 12 or newer
- Xcode 15+ with the macOS SDK

## Installation

```toml
[dependencies]
tabulardata-rs = "0.1.0"
```

```rust,no_run
use tabulardata::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let frame = DataFrame::from_columns(&[
        Column::strings("name", vec![Some("Ada".into()), Some("Grace".into())]),
        Column::ints("score", vec![Some(98), Some(99)]),
    ])?;
    println!("shape = {:?}", frame.shape());
    Ok(())
}
```

## Smoke example

```bash
cargo run --example 01_smoke
```

The smoke example creates two data frames, writes `target/tabular.csv`, reads it back through `TabularData`, performs a left join, and prints the resulting shapes.

## Notes

- `TabularData` is a Swift-only framework, so this crate is implemented through a `SwiftPM` bridge instead of Objective-C headers.
- Typed column access currently supports `String`, `Int`, `Double`, and `Bool` columns. For mixed/summary frames, use `rows_json()`.
- The implementation surface was validated with small Swift probes before the Rust bindings were written.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
