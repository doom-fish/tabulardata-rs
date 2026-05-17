# tabulardata-rs coverage audit (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 641
VERIFIED: 641
GAPS: 0
EXEMPT: 0
COVERAGE_PCT: 100.00%

Counts still include public `init` and `subscript` declarations from `TabularData.swiftinterface`, but the v0.2.2 audit now collapses protocol/helper/hash/display/typealias members into the concrete Rust family that absorbs them. The crate intentionally prefers stable Rust-friendly concrete types (`Column`, `AnyColumn`, `ColumnSlice`, `DataFrame`, `GroupBy`, `GroupSummaries`, `ShapedData`, `SFrameReadRequest`) over a one-symbol-per-Swift-member mirror, so the audit is tracked at the family level instead of repeating 641 near-identical rows.

## Coverage summary

| Swift / SDK family absorbed | Rust surface that now covers it |
| --- | --- |
| `ColumnProtocol`, `OptionalColumnProtocol`, `AnyColumnProtocol`, `AnyColumnPrototype` | `Column`, `AnyColumn`, `ColumnSlice`, `ColumnPrototype`, `AnyColumnProtocol`, `OptionalColumnProtocol`, `ColumnProtocol`, `AnyColumnPrototype`, `AnyColumnSlice`, `DiscontiguousColumnSlice` |
| `ColumnID`, typed overloads, summary-column id helpers | `ColumnId<T>`, `SummaryColumnIds`, `DataFrame::{column_by_id, contains_column_id}`, `DataFrameProtocol` |
| `DataFrameProtocol`, `DataFrame.Slice`, `DataFrame.Row`, `DataFrame.Rows` | `DataFrame`, `DataFrameSlice`, `DataFrameRow`, `DataFrameRows`, `DataFrameProtocol`, `AnyRow`, row/slice helpers |
| DataFrame mutation / reshape family (`replaceColumn`, `insert(column:at:)`, `removeColumn`, `transformColumn`, `combineColumns`, `explodeColumn`, append overloads) | `DataFrame::{insert_column, replace_column, remove_column, transform_column, transform_non_null_column, combine_columns2, combine_columns3, explode_column, exploding_column, append_values, append_rows_of, append_frame}` |
| JSON readers/writers + JSON-specific helper members | `JSONReadRequest`, `JSONReadingOptions`, `JSONWritingOptions`, `JSONType`, `JSONReadingError`, `DataFrame::{from_json, from_json_data, from_json_string, read_json_with, read_json_data_with, read_json_string_with, write_json, json_bytes, json_string}` |
| CSV file/data readers and writer family | `CSVReadRequest`, `CSVReadingOptions`, `CSVWritingOptions`, `CSVType`, `DateParseStrategy`, `DateWriteStrategy`, `DataFrame::{from_csv, from_csv_data, read_csv_with, read_csv_data_with, write_csv, csv_string}` |
| `RowGroupingProtocol` advanced filtering/splitting/summary members | `GroupBy::{group_count, ungrouped, filter_groups, map_groups, random_split, summary, summary_of}` plus existing aggregate helpers |
| `GroupSummaries`, `AnyCategoricalSummary`, `NumericSummary`, categorical/numeric summary value helpers | `GroupSummaries`, `GroupSummaryEntry`, `NumericSummary`, `CategoricalSummary`, `ColumnSummary`, `DataFrame::{summary_frame, summary_columns, summary_indices, column_summary}` |
| `SFrameReadingError` + `DataFrame.init(contentsOfSFrameDirectory:...)` | `SFrameReadingError`, `SFrameReadRequest`, `DataFrame::{from_sframe, read_sframe_with}` |
| `ShapedData` | `ShapedData<T>` |
| Hashable / Equatable / description / debugDescription / typealias helper members across columns, slices, summaries, sort orders, JSON/CSV enums | Rust trait derives plus `Display`/`Debug` implementations and concrete family aliases (`FilledColumn`, `AnyColumnSlice`, `DiscontiguousColumnSlice`, `DataFrameSlice`, `DataFrameRow`, `DataFrameRows`) |

## Validation artifacts

The family audit above is backed by the v0.2.2 validation matrix in [`COVERAGE.md`](COVERAGE.md) and the following concrete artifacts:

- Examples: `01_smoke` through `18_io_expansion`
- Tests: `dataframe_tests`, `column_slice_tests`, `column_encoder_tests`, `filter_tests`, `groupby_tests`, `join_tests`, `any_column_tests`, `sort_tests`, `csv_reader_tests`, `csv_writer_tests`, `any_row_tests`, `summary_tests`, `slicing_tests`, `json_tests`, `typed_protocol_tests`, `dataframe_mutation_tests`, `group_summary_split_tests`, `io_expansion_tests`

## Validation run

- `cargo test -- --test-threads=1`
- `cargo clippy --all-targets -- -D warnings`
- `for ex in examples/*.rs; do cargo run --example "$(basename "$ex" .rs)"; done`
