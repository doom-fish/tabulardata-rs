# tabulardata

Safe, idiomatic Rust bindings for Apple's [TabularData](https://developer.apple.com/documentation/tabulardata) framework on macOS.

## Features

- **Gold-standard Swift bridge** — retained Swift boxes plus `@_cdecl` entry points, following the same bridge style used in `screencapturekit-rs`.
- **`DataFrame` construction** — create frames from typed `Column` values or heterogeneous `AnyRow` rows.
- **Column introspection** — snapshot `AnyColumn` values, derive `ColumnSlice`s, and compute summaries in Rust.
- **Column encoding** — encode/decode columns through typed JSON payloads.
- **Filtering, sorting, slicing, formatting** — materialize row subsets, stable orderings, prefixes/suffixes, formatted descriptions, and column projections.
- **Grouping, splits + joins** — counts, sums, means, quantiles, random/stratified splits, and inner/left/right/full joins.
- **CSV + JSON IO** — configurable CSV/JSON readers and writers, row/column projection, type hints, and in-memory string/byte representations.
- **14 worked examples + 14 tests** — one example and one integration test for each requested logical area.

## Requirements

- macOS 12 or newer
- Xcode 15+ with the macOS SDK

## Installation

```toml
[dependencies]
tabulardata-rs = "0.2.1"
```

```rust,no_run
use tabulardata::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let frame = DataFrame::from_rows(&[
        AnyRow::new()
            .with_value("name", "Ada")
            .with_value("score", 98.0),
        AnyRow::new()
            .with_value("name", "Grace")
            .with_value("score", 99.0),
    ])?;

    let sorted = frame.sorted_by(&[SortKey::descending("score")])?;
    println!("shape = {:?}", sorted.shape());
    Ok(())
}
```

## Examples

| Area | Command |
| --- | --- |
| `DataFrame` | `cargo run --example 01_smoke` |
| `ColumnSlice` | `cargo run --example 02_column_slice_summary` |
| `ColumnEncoder` | `cargo run --example 03_column_encoder_roundtrip` |
| `Filter` | `cargo run --example 04_filter_rows` |
| `GroupBy` | `cargo run --example 05_groupby_counts` |
| `Join` | `cargo run --example 06_join_frames` |
| `AnyColumn` | `cargo run --example 07_any_column_snapshot` |
| `Sort` | `cargo run --example 08_sort_rows` |
| `CSVReader` | `cargo run --example 09_csv_reader_subset` |
| `CSVWriter` | `cargo run --example 10_csv_writer_string` |
| `AnyRow` | `cargo run --example 11_any_row_mutation` |
| `Summary` | `cargo run --example 12_summary_report` |
| `Slicing` | `cargo run --example 13_slicing_rows_and_columns` |
| `JSON` | `cargo run --example 14_json_io` |

## Coverage

See [COVERAGE.md](COVERAGE.md) for the v0.2.x API matrix, example/test mapping, and explicit out-of-scope `TabularData` surfaces.

## Notes

- `TabularData` is a Swift-only framework, so this crate is implemented through a `SwiftPM` bridge instead of Objective-C headers.
- Typed `Column` construction covers `String`, `Int`, `Double`, and `Bool`; heterogeneous APIs use `AnyValue`/`AnyRow`.
- Filtering, sorting, and grouping are described in Rust and executed in Swift through JSON payloads instead of bridged closures.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
