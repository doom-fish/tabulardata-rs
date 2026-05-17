# tabulardata

Safe, idiomatic Rust bindings for Apple's [TabularData](https://developer.apple.com/documentation/tabulardata) framework on macOS.

## Features

- **Gold-standard Swift bridge** — retained Swift boxes plus `@_cdecl` entry points, following the same bridge style used in `screencapturekit-rs`.
- **`DataFrame` construction + mutation** — create frames from typed `Column` values or heterogeneous `AnyRow` rows, then insert/replace/remove/transform/combine/explode columns and append row families.
- **Typed column protocols** — `ColumnId`, `ColumnPrototype`, `ColumnProtocol`, `OptionalColumnProtocol`, `AnyColumnProtocol`, `AnyColumnSlice`, and `DiscontiguousColumnSlice` mirror the Swift family through concrete Rust-friendly APIs.
- **Column introspection + statistics** — snapshot `AnyColumn` values, derive `ColumnSlice`s, compute summaries, and query min/max/argmin/argmax/mean/stddev in Rust.
- **Column encoding** — encode/decode columns through typed JSON payloads.
- **Filtering, sorting, slicing, formatting** — materialize row subsets, stable orderings, prefixes/suffixes, formatted descriptions, and column projections.
- **Grouping, group summaries, splits + joins** — counts, sums, means, quantiles, group-level summaries/filtering/mapping/random splits, and inner/left/right/full joins.
- **CSV + JSON + `SFrame` IO** — configurable CSV/JSON readers and writers, row/column projection, type hints, in-memory string/byte representations, and `SFrame` directory imports.
- **Shaped data helpers** — pure-Rust `ShapedData<T>` mirrors the shaped export family exposed by the framework.
- **18 worked examples + 18 tests** — the expanded v0.2.2 surface ships with dedicated validation across the new families.

## Requirements

- macOS 12 or newer
- Xcode 15+ with the macOS SDK

## Installation

```toml
[dependencies]
tabulardata-rs = "0.2.2"
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
| `TypedProtocols` | `cargo run --example 15_typed_protocols` |
| `DataFrameMutation` | `cargo run --example 16_dataframe_mutation` |
| `GroupSummarySplit` | `cargo run --example 17_group_summary_split` |
| `IOExpansion` | `cargo run --example 18_io_expansion` |

## Coverage

See [COVERAGE.md](COVERAGE.md) for the v0.2.2 API matrix and [COVERAGE_AUDIT.md](COVERAGE_AUDIT.md) for the family-level 100% audit summary.

## Notes

- `TabularData` is a Swift-only framework, so this crate is implemented through a `SwiftPM` bridge instead of Objective-C headers.
- Typed `Column` construction covers `String`, `Int`, `Double`, `Bool`, `Date`, and `Data`; heterogeneous APIs use `AnyValue`/`AnyRow`.
- Filtering, sorting, and grouping are described in Rust and executed in Swift through JSON payloads instead of bridged closures; higher-level mutation and group-summary helpers are composed from those primitives in Rust.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
