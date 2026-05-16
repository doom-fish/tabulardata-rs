# tabulardata-rs v0.2.0 coverage

This document tracks the public Rust API shipped in `v0.2.0`, the matching Swift bridge area, and the validation artifacts that exercise it. Status meanings:

- ✅ implemented and covered by at least one example and one test
- ➖ intentionally out of scope for `v0.2.0`

## Requested logical areas

| Area | Rust surface | Swift bridge | Example | Test | Status | Notes |
| --- | --- | --- | --- | --- | --- | --- |
| DataFrame | `DataFrame::{new, from_columns, from_csv, shape, row_count, column_count, column_names, append_column, rename_column, column, rows_json, summary, joined, write_csv}` | `Core.swift`, `DataFrame.swift` | `01_smoke` | `dataframe_tests` | ✅ | Core frame lifecycle, typed columns, summaries, joins, CSV IO. |
| ColumnSlice | `DataFrame::{column_slice, column_mask}`, `ColumnSlice::{new, len, is_empty, value, range, distinct, summary}` | `Core.swift`, `ColumnSlice.swift` | `02_column_slice_summary` | `column_slice_tests` | ✅ | Supports range and mask materialization plus Rust-side summarization. |
| ColumnEncoder | `DataFrame::{encode_column, decode_column}`, `ColumnCodec`, `ColumnElementType` | `Core.swift`, `ColumnEncoder.swift` | `03_column_encoder_roundtrip` | `column_encoder_tests` | ✅ | JSON payload bridge for typed column serialization. |
| Filter | `Filter::{eq, ne, gt, gte, lt, lte, between, one_of, contains, is_null, is_not_null, and, or, negate}`, `DataFrame::filtered` | `Core.swift`, `Filter.swift` | `04_filter_rows` | `filter_tests` | ✅ | Rust builds filter specs; Swift evaluates them row-by-row. |
| GroupBy | `DataFrame::{group_by, group_by_time}`, `GroupBy::{counts, sums, means, quantiles, minimums, maximums, group}`, `TimeUnit`, `GroupValueType` | `Core.swift`, `GroupBy.swift`, `Sort.swift` | `05_groupby_counts` | `groupby_tests` | ✅ | Covers column grouping, time grouping, aggregate frames, and specific group lookup. |
| Join | `JoinColumns::{new, same}`, `DataFrame::joined_on`, `JoinKind` | `Core.swift`, `Join.swift` | `06_join_frames` | `join_tests` | ✅ | Supports inner/left/right/full joins with same or different key names. |
| AnyColumn | `AnyValue::{as_str, as_f64, stable_key, equals, partial_cmp, contains}`, `AnyColumn::{new, len, is_empty, value, slice, mask, distinct, summary}`, `DataFrame::{any_column, any_columns}` | `Core.swift`, `AnyColumn.swift` | `07_any_column_snapshot` | `any_column_tests` | ✅ | Heterogeneous snapshots used by rows, grouping, filtering, and summaries. |
| Sort | `SortKey::{ascending, descending}`, `SortOrder`, `DataFrame::{sorted_by, sort_by}` | `Core.swift`, `Sort.swift` | `08_sort_rows` | `sort_tests` | ✅ | Stable multi-key sorting described in Rust and executed in Swift. |
| CSVReader | `CSVReadRequest::{new, with_columns, with_rows, with_type_hint}`, `CSVType`, `DateParseStrategy`, `DataFrame::read_csv_with` | `Core.swift`, `CSVReader.swift` | `09_csv_reader_subset` | `csv_reader_tests` | ✅ | Covers projections, row windows, delimiter configuration, and type hints. |
| CSVWriter | `CSVWritingOptions::{new, with_includes_header, with_date_strategy, with_nil_encoding, with_true_encoding, with_false_encoding, with_newline, with_delimiter}`, `DateWriteStrategy`, `DataFrame::csv_string` | `Core.swift`, `CSVWriter.swift` | `10_csv_writer_string` | `csv_writer_tests` | ✅ | Covers file and string output paths. |
| AnyRow | `AnyRow::{new, with_value, insert, get, len, is_empty}`, `DataFrame::{from_rows, row, rows, append_row, insert_row, replace_row}` | `Core.swift`, `AnyRow.swift` | `11_any_row_mutation` | `any_row_tests` | ✅ | Builds typed empty frames from row payloads before mutation/appends. |
| Summary | `DataFrame::{summary_frame, summary_columns, summary_indices, column_summary}`, `NumericSummary`, `CategoricalSummary`, `ColumnSummary` | `Core.swift`, `Summary.swift` | `12_summary_report` | `summary_tests` | ✅ | Mixes framework summary frames with Rust-friendly per-column summaries. |
| Slicing | `DataFrame::{slice_rows, prefix_rows, suffix_rows, select_columns}` | `Core.swift`, `Slicing.swift` | `13_slicing_rows_and_columns` | `slicing_tests` | ✅ | Row-range slicing and column projection helpers. |

## Supporting public Rust types

| Type / API | Status | Notes |
| --- | --- | --- |
| `ColumnData::{Strings, Ints, Doubles, Bools}` | ✅ | Typed column construction used by `DataFrame::from_columns`. |
| `Column::{strings, ints, doubles, bools, len, is_empty}` | ✅ | Validated by `01_smoke` and `dataframe_tests`. |
| `CSVReadingOptions::{new, with_has_header_row, with_nil_encodings, with_true_encodings, with_false_encodings, with_floating_point_type, with_date_parse_strategy, with_delimiter, with_escape_character, with_ignores_empty_lines, with_uses_quoting, with_uses_escaping}` | ✅ | Used directly and via `CSVReadRequest`. |
| `TabularDataError` | ✅ | Shared error surface for all bridged operations. |
| Prelude re-exports (`tabulardata::prelude::*`) | ✅ | Exercised by README, examples, and tests. |

## Explicitly out of scope in v0.2.0

These TabularData SDK surfaces were audited but intentionally not wrapped in this release. They can be added in a later wave without breaking the current API.

| SDK surface | Status | Reason |
| --- | --- | --- |
| JSON IO (`JSONType`, `JSONReadingOptions`, `JSONWritingOptions`, JSON readers/writers) | ➖ | The requested v0.2.0 scope focused on CSV and DataFrame operations. |
| `SFrameReadingError` / SFrame import APIs | ➖ | Not part of the requested logical areas. |
| `ShapedData` / shaped exports | ➖ | Not requested and would require a separate Rust model. |
| Generic Swift protocols (`ColumnProtocol`, `OptionalColumnProtocol`, `AnyColumnProtocol`, `AnyColumnPrototype`, `DataFrameProtocol`, `RowGroupingProtocol`) | ➖ | Rust binds concrete operations instead of mirroring Swift protocol hierarchies. |
| Formatting and display helpers (`FormattingOptions`, string interpolation/display-only APIs) | ➖ | Presentation helpers were not part of the requested bridge expansion. |
| Random/stratified split helpers | ➖ | Useful future addition, but outside the 13-area target. |
| Swift-only generic column math and direct typed-column mutation APIs beyond the existing `Column` constructors | ➖ | The Rust surface intentionally centers on stable JSON/box-based interop. |

## Validation run for this matrix

The v0.2.0 matrix above was validated with:

- `cargo test`
- `cargo clippy --all-targets -- -D warnings`
- `for ex in examples/*.rs; do cargo run --example "$(basename "$ex" .rs)"; done`
